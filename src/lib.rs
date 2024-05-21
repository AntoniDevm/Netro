use anyhow::bail;
use libc::{sendto, sockaddr, sockaddr_ll};
use nix::errno::Errno;
use nix::sys::socket::{
    recvfrom, socket, AddressFamily, LinkAddr, SockFlag, SockProtocol, SockType,
};
use std::mem;
use std::os::fd::{AsRawFd, OwnedFd};
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
pub const MTU: usize = 1500;

pub struct Socket {
    /// The socket
    fd: Arc<OwnedFd>,
    /// The buffer that will be passed to the `recvfrom()` function.
    /// It's size is the MTU constant or 1500 bytes
    packet_buff: Arc<Mutex<[u8; MTU]>>,
}

impl Socket {
    /// Creates a Raw Packet Socket with no flags
    pub fn new() -> anyhow::Result<Self> {
        todo!("Do Something about the problem with the one packet buffer. It will take ages to recieve a packet if many function are in line to write to the buffer but only one can!!");
        let socket = socket(
            AddressFamily::Packet,
            SockType::Raw,
            SockFlag::empty(),
            Some(SockProtocol::EthAll),
        )?;
        Ok(Self {
            fd: Arc::new(socket),
            packet_buff: Arc::new(Mutex::new([0u8; MTU])),
        })
    }
    /// Recvies data from the socket by calling recvfrom
    pub fn recv(&mut self) -> tokio::task::JoinHandle<Result<(usize, Option<LinkAddr>), Errno>> {
        self.clear_packet();
        let fd = Arc::clone(&self.fd);
        let buffer = Arc::clone(&self.packet_buff);
        let handle = tokio::spawn(async move {
            let mut buff = buffer.lock().await;
            recvfrom::<LinkAddr>(fd.as_raw_fd(), &mut *buff)
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

    fn clear_packet(&mut self) {
        self.packet_buff = Arc::new(Mutex::new([0u8; MTU]));
    }

    pub async fn get_packet(&self) -> MutexGuard<[u8; MTU]> {
        self.packet_buff.lock().await
    }
}
