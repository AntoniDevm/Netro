pub mod ip;


use ip::ipv4::IPv4;
#[derive(Debug)]
pub enum Network {
    IPv4(IPv4),
    IPv6,
}

