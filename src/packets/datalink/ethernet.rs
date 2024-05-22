use nom::{bytes::complete::take, number::complete::be_u16};

use crate::packets::Parsable;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Ethernet {
    destination: Mac,
    source: Mac,
    ethertype: EtherType,
}

impl Parsable for Ethernet {
    fn parse(source: &[u8]) -> nom::IResult<&[u8], Self>
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

#[derive(Debug)]
pub enum EtherType {
    /// 0x0800
    IPv4,
    /// 0x0806
    ARP,
    /// 0x86DD
    IPv6
}

impl Parsable for EtherType {
    fn parse(source: &[u8]) -> nom::IResult<&[u8], Self>
        where
            Self: Sized {
        let (rem,code) = be_u16(source)?;
        let ethertype = match code {
            0x0800 => Self::IPv4,
            0x0806 => Self::ARP,
            0x86DD => Self::IPv6,
            _ => panic!("EtherType not supported {}",code),
        };
        Ok((rem,ethertype))        
    }
}

#[derive(Debug)]
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
    fn parse(source: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let raw = take(6usize)(source)?;
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
