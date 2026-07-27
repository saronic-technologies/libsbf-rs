use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

// GPSIon Block 5893
#[binrw]
#[derive(Clone, Debug)]
pub struct GPSIon {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub reserved: u8,
    pub alpha_0: f32,
    pub alpha_1: f32,
    pub alpha_2: f32,
    pub alpha_3: f32,
    pub beta_0: f32,
    pub beta_1: f32,
    pub beta_2: f32,
    pub beta_3: f32,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
