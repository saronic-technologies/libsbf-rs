use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

// SatVisibility Block 4012
#[binrw]
#[derive(Clone, Debug)]
pub struct SatVisibility {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = binrw_util::unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = binrw_util::wrap_subblocks)]
    pub satellites: Vec<SatInfo>,
}

// SatInfo sub-block
#[binrw]
#[derive(Clone, Debug)]
pub struct SatInfo {
    pub svid: u8,
    /// GLONASS frequency number with an offset of 8, from 1 to 21; reserved otherwise.
    #[br(map = binrw_util::map_u1_zero)]
    #[bw(map = binrw_util::unmap_u1_zero)]
    pub freq_nr: Option<u8>,
    /// Azimuth in 0.01 degrees, 0 is North and increasing towards East.
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub azimuth: Option<u16>,
    /// Elevation in 0.01 degrees relative to the local horizontal plane.
    #[br(map = binrw_util::map_i2)]
    #[bw(map = binrw_util::unmap_i2)]
    pub elevation: Option<i16>,
    #[br(map = |x: u8| RiseSet::from(x))]
    #[bw(map = |x: &RiseSet| u8::from(*x))]
    pub rise_set: RiseSet,
    /// Source of the visibility info: 1 almanac, 2 ephemeris, 255 unknown.
    pub satellite_info: u8,
}

/// Rise/set state of a satellite.
#[derive(Clone, Copy, Debug)]
pub enum RiseSet {
    Setting,
    Rising,
    Unknown,
    Other(u8),
}

impl From<u8> for RiseSet {
    fn from(value: u8) -> Self {
        match value {
            0 => RiseSet::Setting,
            1 => RiseSet::Rising,
            // 3 in the 2-bit ChannelStatus field, 255 in SatVisibility.
            3 | 255 => RiseSet::Unknown,
            other => RiseSet::Other(other),
        }
    }
}

impl From<RiseSet> for u8 {
    fn from(value: RiseSet) -> Self {
        match value {
            RiseSet::Setting => 0,
            RiseSet::Rising => 1,
            RiseSet::Unknown => 255,
            RiseSet::Other(other) => other,
        }
    }
}
