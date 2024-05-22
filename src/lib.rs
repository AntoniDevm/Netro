pub mod packet_memory;
pub mod packets;

use anyhow::bail;
use libc::{sendto, sockaddr, sockaddr_ll};
use nix::errno::Errno;
use nix::sys::socket::{
    recvfrom, socket, AddressFamily, LinkAddr, SockFlag, SockProtocol, SockType,
};
use packet_memory::BufferPool;
use std::mem;
use std::os::fd::{AsRawFd, OwnedFd};
use std::sync::Arc;
use tokio::sync::Mutex;
pub const MTU: usize = 1500;

pub struct Socket {
    /// The socket
    fd: Arc<OwnedFd>,
    /// The buffer that will be passed to the `recvfrom()` function.
    /// It's size is the MTU constant or 1500 bytes
    buff_pool: Arc<Mutex<BufferPool>>,
}
impl Socket {
    /// Creates a Raw Packet Socket with no flags
    pub fn new() -> anyhow::Result<Self> {
        let socket = socket(
            AddressFamily::Packet,
            SockType::Raw,
            SockFlag::empty(),
            Some(SockProtocol::EthAll),
        )?;
        Ok(Self {
            fd: Arc::new(socket),
            buff_pool: Arc::new(Mutex::new(BufferPool::new(0, MTU as u32))),
        })
    }
    /// Recvies data from the socket by calling recvfrom
    pub fn recv(&mut self) -> tokio::task::JoinHandle<Result<(Vec<u8>,usize), Errno>> {
        let fd = Arc::clone(&self.fd);
        let buffer_pool = Arc::clone(&self.buff_pool);
        let handle = tokio::spawn(async move {
            let mut buf_pool = buffer_pool.lock().await;
            let mut buffer = buf_pool.get();
            let ret = recvfrom::<LinkAddr>(fd.as_raw_fd(), &mut buffer)?;
            Ok((buffer,ret.0))
        });
        handle
    }

    pub async fn send(&mut self, data: &[u8]) -> anyhow::Result<isize> {
        let address = sockaddr_ll {
            sll_family: 17,
            sll_ifindex: 2,
            sll_protocol: 0x0011,
            sll_addr: [0; 8], // Useless
            sll_halen: 0,     // Useless
            sll_hatype: 0,    // Useless
            sll_pkttype: 0,   // Useless
        };

        let size = unsafe {
            sendto(
                self.fd.as_raw_fd(),
                data.as_ptr().cast(),
                mem::size_of_val(data),
                0,
                &address as *const _ as *const sockaddr,
                20,
            )
        };
        if size < 0 {
            bail!(std::io::Error::last_os_error())
        }
        Ok(size)
    }

    
}
