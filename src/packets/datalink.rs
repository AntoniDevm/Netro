
pub mod ethernet;
use ethernet::Ethernet;

pub enum DataLink {
    Ethernet(Ethernet)
}



