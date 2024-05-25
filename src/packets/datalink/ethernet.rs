use crate::packets::Parsable;
use anyhow::bail;
use nom::{bytes::complete::take, number::complete::be_u16, Finish};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub struct Ethernet {
    pub destination: Mac,
    pub source: Mac,
    pub ethertype: EtherType,
}

impl Parsable for Ethernet {
    fn parse(source: &[u8]) -> anyhow::Result<(&[u8], Self)>
    where
        Self: Sized,
    {
        let (rem, destination) = Mac::parse(source)?;
        let (rem, source) = Mac::parse(rem)?;
        let (rem, ethertype) = EtherType::parse(rem)?;
        Ok((
            rem,
            Ethernet {
                destination,
                source,
                ethertype,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EtherType {
    /// 0x0800
    IPv4,
    /// 0x0806
    ARP,
    /// 0x86DD
    IPv6,
}

impl Parsable for EtherType {
    fn parse(source: &[u8]) -> anyhow::Result<(&[u8], Self)>
    where
        Self: Sized,
    {
        let (rem, code) = match be_u16::<&[u8], nom::error::Error<&[u8]>>(source) {
            Ok(r) => r,
            Err(er) => {
                bail!("Error parsing ethertype: {:?}", er);
            }
        };
        let ethertype = match code {
            0x0800 => Self::IPv4,
            0x0806 => Self::ARP,
            0x86DD => Self::IPv6,
            _ => panic!("EtherType not supported {}", code),
        };
        Ok((rem, ethertype))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub struct Mac {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
}

impl Parsable for Mac {
    fn parse(source: &[u8]) -> anyhow::Result<(&[u8], Self)>
    where
        Self: Sized,
    {
        let raw = match take::<usize, &[u8], nom::error::Error<&[u8]>>(6)(source).finish() {
            Ok(r) => r,
            Err(er) => {
                bail!("Error parsing mac address. Error: {:?}", er)
            }
        };
        let buff = raw.1;
        let mac = Mac {
            a: buff[0],
            b: buff[1],
            c: buff[2],
            d: buff[3],
            e: buff[4],
            f: buff[5],
        };
        Ok((raw.0, mac))
    }
}

#[cfg(test)]
mod mac_testing {
    use crate::packets::Parsable;

    use super::Mac;
    use rand::Rng;
    #[test]
    fn forloop() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let a = rng.gen();
            let b = rng.gen();
            let c = rng.gen();
            let d = rng.gen();
            let e = rng.gen();
            let f = rng.gen();

            let mac1 = Mac { a, b, c, d, e, f };
            let mac2 = Mac::parse(&[a, b, c, d, e, f]).unwrap().1;
            assert_eq!(mac1, mac2, "Parsing {:?} and {:?}", mac1, mac2);
        }
    }
}

#[cfg(test)]
mod ether_testing {
    use crate::packets::Parsable;
    use pnet::{
        self,
        packet::ethernet::{EtherTypes, MutableEthernetPacket},
    };
    use rand::Rng;

    use super::{Ethernet, Mac};
    #[test]
    fn pnet_testing() {
        let mut rng = rand::thread_rng();
        let source_mac = [rng.gen(); 6];
        let dest_mac = [rng.gen(); 6];
        let mut buff = [0u8; 42];
        let mut packet = MutableEthernetPacket::new(&mut buff).unwrap();
        packet.set_source(source_mac.into());
        packet.set_destination(dest_mac.into());
        packet.set_ethertype(EtherTypes::Ipv4);

        let (_rem, ethernet) = Ethernet::parse(&buff).unwrap();

        assert_eq!(
            ethernet,
            Ethernet {
                destination: Mac::parse(&dest_mac).unwrap().1,
                source: Mac::parse(&source_mac).unwrap().1,
                ethertype: crate::packets::datalink::ethernet::EtherType::IPv4
            }
        )
    }
}
