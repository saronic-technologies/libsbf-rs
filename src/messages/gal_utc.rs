use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_u2, map_u4, map_f4, map_f8, unmap_u2, unmap_u4, unmap_f4, unmap_f8, write_vec};

// GALUtc Block 4031
#[binrw]
#[derive(Debug, Clone)]
pub struct GALUtc {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub source: u8,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub a_1: Option<f32>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub a_0: Option<f64>,
    pub t_ot: u32,
    pub wn_ot: u8,
    pub del_t_ls: i8,
    pub wn_lsf: u8,
    pub dn: u8,
    pub del_t_lsf: i8,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = write_vec)]
    pub padding: Vec<u8>,
}

impl GALUtc {
    // Source constants
    pub const SOURCE_INAV: u8 = 2;
    pub const SOURCE_FNAV: u8 = 16;
}