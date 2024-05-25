pub mod datalink;
pub mod network;

use anyhow::bail;
use datalink::DataLink;

use self::{datalink::ethernet::EtherType, network::{ip::ipv4::IPv4, Network}};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Packet {
    /// Layer 2 from the osi model
    datalink: Option<DataLink>,

    /// Layer 3 from the osi mode
    network: Option<Network>
}

impl Parsable for Packet {
    fn parse(source: &[u8]) -> anyhow::Result<(&[u8],Self)> 
    where
        Self: Sized,
    {
        // It's not a good idea to parse
        // layers. Since many times the previous layer specified the next protocol it a good idea
        // to parse with the protocol specified
        let (payload,datalink) = DataLink::parse(source)?;
        let (payload,network) = match &datalink {
            DataLink::Ethernet(eth) => {
                if eth.ethertype == EtherType::IPv4 {
                    let (payload,ipv4) = IPv4::parse(&payload)?;
                    (payload, Network::IPv4(ipv4))
                } else {
                    bail!("Unsupported EtherType")
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                    bail!("Unsupported Datalink protocol")
            }
        };
        Ok((
            payload, // Remaining bytes
            Self {
                datalink: Some(datalink),
                network: Some(network)
            },
        ))
    }
}

pub trait Parsable {
    fn parse(source: &[u8]) -> anyhow::Result<(&[u8], Self)> 
    where
        Self: Sized;
}
