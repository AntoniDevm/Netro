pub mod packets;
pub mod errors;

use anyhow::bail;
use libc::{ioctl, sendto, sockaddr, sockaddr_ll, FIONREAD};
use nix::errno::Errno;
use nix::sys::socket::{
    recvfrom, socket, AddressFamily, LinkAddr, SockFlag, SockProtocol, SockType,
};
use packets::{Packet, Parsable};
use std::os::fd::{AsRawFd, OwnedFd};
use std::sync::Arc;
use std::mem;
pub const MTU: usize = 1500;

pub struct Socket {
    /// The socket
    fd: Arc<OwnedFd>,
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
        })
    }
    /// Recvies data from the socket by calling recvfrom
    pub fn recv(&mut self) -> tokio::task::JoinHandle<Result<Packet, Errno>> {
        let fd = Arc::clone(&self.fd);
        let handle = tokio::spawn(async move {
            let mut size = 0;
            let mut code = unsafe { ioctl(fd.as_raw_fd(), FIONREAD, &mut size) };
            while size == 0 {
                code = unsafe { ioctl(fd.as_raw_fd(), FIONREAD, &mut size) };
            }
            if code < 0 {
                return Err(Errno::last())
            }
            let mut buffer = vec![0u8; size as usize];
            let ret = recvfrom::<LinkAddr>(fd.as_raw_fd(), &mut buffer).unwrap();
            let (_rem, pac) = Packet::parse(&buffer[..ret.0]).unwrap();
            Ok(pac)
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
