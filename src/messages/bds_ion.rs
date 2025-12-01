use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_u2, map_u4, unmap_u2, unmap_u4, write_vec};

// BDSIon Block 4120
#[binrw]
#[derive(Debug, Clone)]
pub struct BDSIon {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
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
    #[bw(write_with = write_vec)]
    pub padding: Vec<u8>,
}