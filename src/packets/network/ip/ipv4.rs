use std::net::Ipv4Addr;

use anyhow::bail;
use nom::{
    bits::{bits, complete::take as take_bits},
    combinator::{map, value, verify},
    error::{Error, VerboseError},
    number::{
        complete::{be_u8, u16 as pu16},
        Endianness,
    },
    sequence::{pair, tuple},
    streaming::bool as bbool,
    Err,
};

use crate::Parsable;

use super::Version;
#[derive(Debug)]
#[allow(unused)]
pub struct IPv4 {
    version: Version,
    ihl: u8,
    hscp: u8,
    enc: u8,
    total_length: u16,
    identification: u16,
    flags: Flags,
    frag_offset: u16,
    ttl: u8,
    protocol: Protocol,
    checksum: u16,
    source: Ipv4Addr,
    destination: Ipv4Addr
}

impl Parsable for IPv4 {
    fn parse(source: &[u8]) -> anyhow::Result<(&[u8], Self)>
    where
        Self: Sized,
    {
        let (input, (version, ihl)) = version_ihl()(source).unwrap();
        let (input, (hscp, enc)) = hscp_enc()(input).unwrap();
        let (input, total_length) = total_length()(input).unwrap();
        let (input, identification) = identification()(input).unwrap();
        let (input, ((r, df, mf), frag_offset)) = flags_offset()(input).unwrap();
        assert_eq!(r, false); // Remove if first bit has/gets a use
        let (input, ttl) = ttl()(input).unwrap();
        let (input, protocol) = protocol()(input).unwrap();
        let (input, checksum) = checksum()(input).unwrap();
        let (input, (a,b,c,d)) = ipv4()(input).unwrap();
        let (input, (e,f,g,h)) = ipv4()(input).unwrap();
        Ok((
            input,
            Self {
                version,
                ihl,
                hscp,
                enc,
                total_length,
                identification,
                flags: Flags::new(df, mf),
                frag_offset,
                ttl,
                protocol: Protocol::try_from(protocol)?,
                checksum,
                source: Ipv4Addr::new(a, b, c, d),
                destination: Ipv4Addr::new(e, f, g, h)
            },
        ))
    }
}

fn ipv4<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], (u8,u8,u8,u8)), Err<Error<&[u8]>>> {
    tuple((be_u8,be_u8,be_u8,be_u8))
}

fn checksum<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], u16), Err<Error<&[u8]>>> {
    pu16(Endianness::Big)
}

fn protocol<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], u8), Err<Error<&[u8]>>> {
    be_u8
}

fn ttl<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], u8), Err<Error<&[u8]>>> {
    be_u8
}

fn identification<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], u16), Err<Error<&[u8]>>> {
    pu16(Endianness::Big)
}

fn version_ihl<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], (Version, u8)), Err<Error<&[u8]>>> {
    bits::<_, _, Error<(&[u8], usize)>, Error<_>, _>(pair(
        value(
            Version::V4,
            verify(take_bits::<_, u8, _, _>(4usize), |v| v == &4),
        ),
        take_bits::<_, u8, _, _>(4usize),
    ))
}

/// Can be optimized to return an enum. But I can't be bothered :D
fn hscp_enc<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], (u8, u8)), Err<Error<&[u8]>>> {
    bits::<_, _, Error<(&[u8], usize)>, Error<_>, _>(pair(
        take_bits::<_, u8, _, _>(6usize),
        take_bits::<_, u8, _, _>(2usize),
    ))
}
fn total_length<'a>() -> impl FnMut(&'a [u8]) -> Result<(&[u8], u16), Err<Error<&[u8]>>> {
    pu16(Endianness::Big)
}

fn flags_offset<'a>(
) -> impl FnMut(&'a [u8]) -> Result<(&[u8], ((bool, bool, bool), u16)), Err<VerboseError<&[u8]>>> {
    bits::<&[u8], _, _, _, _>(pair(
        tuple::<_, _, VerboseError<_>, _>((bbool, bbool, bbool)),
        take_bits(13usize),
    ))
}
#[derive(Debug)]
pub enum Protocol {
    ICMP = 1,
    IGMP = 2,
    GGP = 3,
    IPinIP = 4,
    ST = 5,
    TCP = 6,
    CBT = 7,
    EGP = 8,
    IGP = 9,
    BBNRCCMON = 10,
    NVPII = 11,
    PUP = 12,
    ARGUS = 13,
    EMCON = 14,
    XNET = 15,
    CHAOS = 16,
    UDP = 17,
    MUX = 18,
    DCNMEAS = 19,
    HMP = 20,
}

impl TryFrom<u8> for Protocol {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::ICMP,
            2 => Self::IGMP,
            3 => Self::GGP,
            4 => Self::IPinIP,
            5 => Self::ST,
            6 => Self::TCP,
            7 => Self::CBT,
            8 => Self::EGP,
            9 => Self::IGP,
            10 => Self::BBNRCCMON,
            11 => Self::NVPII,
            12 => Self::PUP,
            13 => Self::ARGUS,
            14 => Self::EMCON,
            15 => Self::XNET,
            16 => Self::CHAOS,
            17 => Self::UDP,
            18 => Self::MUX,
            19 => Self::DCNMEAS,
            20 => Self::HMP,
            _ => bail!("Unimplemented protocol")
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Flags {
    dont_fragment: bool,
    more_fragments: bool,
}

impl Flags {
    pub fn new(df: bool, mf: bool) -> Self {
        Self {
            dont_fragment: df,
            more_fragments: mf,
        }
    }
}

#[cfg(test)]
mod ipv4tests {

    use crate::packets::network::ip::Version;

    use super::flags_offset;
    use super::hscp_enc;
    use super::ttl;
    use super::version_ihl;
    #[test]
    fn ttl_testing() {
        assert_eq!(ttl()(&[0b0000_1010]), Ok(([].as_slice(), 10)));
        assert_eq!(ttl()(&[0b0001_1010, 0x1]), Ok(([0x1].as_slice(), 26)));
        assert_eq!(
            ttl()(&[0b0100_1010, 0x1, 0x2]),
            Ok(([0x1, 0x2].as_slice(), 74))
        );
    }

    #[test]
    fn flags_offset_testing() {
        assert_eq!(
            flags_offset()(&[0b001_00000, 0b00000000]),
            Ok(([].as_slice(), ((false, false, true), 0)))
        );
        assert_eq!(
            flags_offset()(&[0b101_00000, 0b00000100, 0x1]),
            Ok(([0x1].as_slice(), ((true, false, true), 4)))
        );
        assert_eq!(
            flags_offset()(&[0b011_00000, 0b00001000]),
            Ok(([].as_slice(), ((false, true, true), 8)))
        );
        assert_eq!(
            flags_offset()(&[0b000_00000, 0b00001000]),
            Ok(([].as_slice(), ((false, false, false), 8)))
        );
    }

    #[test]
    fn version_ihl_testing() {
        assert_eq!(
            version_ihl()(&[0b0100_0101]).unwrap(),
            ([].as_slice(), (Version::V4, 5u8))
        );
        assert_eq!(
            version_ihl()(&[0b0100_0101, 0x1]).unwrap(),
            ([0x1].as_slice(), (Version::V4, 5u8))
        );
        assert_eq!(
            version_ihl()(&[0b0100_0101, 0x2]).unwrap(),
            ([0x2].as_slice(), (Version::V4, 5u8))
        );
        assert_eq!(
            version_ihl()(&[0b0100_0101, 0x2, 3, 4, 5, 6]).unwrap(),
            ([2, 3, 4, 5, 6].as_slice(), (Version::V4, 5u8))
        );
    }

    #[test]
    fn hscp_enc_testing() {
        assert_eq!(
            hscp_enc()([0b001100_01, 0x13].as_slice()).unwrap(),
            ([0x13].as_slice(), (12, 1)),
        );
        assert_eq!(
            hscp_enc()([0b000010_10].as_slice()).unwrap(),
            ([].as_slice(), (2, 2)),
        );
    }
}
