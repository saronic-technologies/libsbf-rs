use crate::util::binrw::{map_u2, map_u4, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;

// GALGstGps Block 4032
#[binrw]
#[derive(Clone, Debug)]
pub struct GALGstGps {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub source: u8,
    pub a_1g: f32,
    pub a_0g: f32,
    pub t_og: u32,
    pub wn_og: u8,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl GALGstGps {
    // Source constants
    pub const SOURCE_INAV: u8 = 2;
    pub const SOURCE_FNAV: u8 = 16;
}
