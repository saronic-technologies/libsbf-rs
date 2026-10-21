use super::common::RiseSet;
use crate::util::{
    binrw::{
        map_i2, map_u1_enum, map_u1_zero, map_u2, map_u4, unmap_i2, unmap_u1_enum, unmap_u1_zero,
        unmap_u2, unmap_u4, unwrap_subblocks, wrap_subblocks,
    },
    signal::glonass_frequency_number,
};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// SatVisibility Block 4012
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct SatVisibility {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = wrap_subblocks)]
    pub satellites: Vec<SatInfo>,
}

// SatInfo sub-block
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct SatInfo {
    pub svid: u8,
    /// GLONASS frequency number with an offset of 8, from 1 to 14 for satellites launched
    /// after 2005 and up to 21 before that (supported in firmware prior to 1.5.x); reserved
    /// otherwise.
    #[br(map = map_u1_zero)]
    #[bw(map = unmap_u1_zero)]
    pub freq_nr: Option<u8>,
    /// Azimuth in 0.01 degrees, 0 is North and increasing towards East.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub azimuth: Option<u16>,
    /// Elevation in 0.01 degrees relative to the local horizontal plane.
    #[br(map = map_i2)]
    #[bw(map = unmap_i2)]
    pub elevation: Option<i16>,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub rise_set: RiseSet,
    /// Source of the visibility info: 1 almanac, 2 ephemeris, 255 unknown.
    pub satellite_info: u8,
}

impl SatInfo {
    /// GLONASS frequency number, from -7 to 6 for satellites launched after 2005 and up to 13
    /// before that (supported in firmware prior to 1.5.x), None when the receiver reports 0.
    /// Only GLONASS satellites set it; for other satellites the value is reserved.
    pub fn frequency_number(&self) -> Option<i8> {
        self.freq_nr.map(glonass_frequency_number)
    }
}
