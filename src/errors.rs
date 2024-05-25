use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum PacketError {
    ProtocolNotSupported
}


impl Error for PacketError {}

impl Display for PacketError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}",self)
        }
}
