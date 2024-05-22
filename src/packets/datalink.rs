
pub mod ethernet;
use ethernet::Ethernet;

use super::Parsable;
#[derive(Debug)]
pub enum DataLink {
    Ethernet(Ethernet)
}

impl Parsable for DataLink {
    fn parse(source: &[u8]) -> nom::IResult<&[u8], Self>
        where
            Self: Sized 
    {
        // I Must identify what datalink protocol is used 
        let (rem, ethernet ) = Ethernet::parse(source)?;
        Ok((rem,DataLink::Ethernet(ethernet)))
    }
}

