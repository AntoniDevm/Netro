pub mod datalink;
use datalink::DataLink;
use nom::IResult;

#[derive(Debug)]
pub struct Packet {
    /// Layer 2 from the osi model
    datalink: Option<DataLink>,
}

impl Parsable for Packet {
    fn parse(source: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let datalink = DataLink::parse(source).unwrap();
        Ok((
            datalink.0, // Remaining bytes
            Self {
                datalink: Some(datalink.1),
            },
        ))
    }
}

pub trait Parsable {
    fn parse(source: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}
