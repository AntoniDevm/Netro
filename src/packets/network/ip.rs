use crate::packets::Parsable;
pub mod ipv4;

#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    V4,
    V6
}

impl Parsable for Version {
    /// Assumes that you give it 4 bits
    fn parse(source: &[u8]) -> anyhow::Result<(&[u8], Self)> 
        where
            Self: Sized 
    {
        // Example data: 
        // 0100 1100
        //  V4   IHL
        // 0110 0110
        //  V6   IHL
        unimplemented!()
    }
}
