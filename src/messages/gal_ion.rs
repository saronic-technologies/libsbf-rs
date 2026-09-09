use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use bitflags::bitflags;

bitflags! {
    /// StormFlags bit field of the [`GALIon`] block: the five ionospheric
    /// storm flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GALIonStormFlags: u8 {
        const SF5 = 1 << 0;
        const SF4 = 1 << 1;
        const SF3 = 1 << 2;
        const SF2 = 1 << 3;
        const SF1 = 1 << 4;
    }
}

// GALIon Block 4030
#[binrw]
#[derive(Clone, Debug)]
pub struct GALIon {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub source: u8,
    pub a_i0: f32,
    pub a_i1: f32,
    pub a_i2: f32,
    #[br(map = |x: u8| GALIonStormFlags::from_bits_retain(x))]
    #[bw(map = |x: &GALIonStormFlags| x.bits())]
    pub storm_flags: GALIonStormFlags,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl GALIon {
    // Source constants
    pub const SOURCE_INAV: u8 = 2;
    pub const SOURCE_FNAV: u8 = 16;
}
