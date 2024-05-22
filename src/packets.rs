
pub mod datalink;
use datalink::DataLink;

pub struct Packet {
    /// Layer 2 from the osi model
    datalink: Option<DataLink>

}



