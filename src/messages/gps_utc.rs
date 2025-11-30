use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_u2, map_u4, unmap_u2, unmap_u4, write_vec};

// GPSUtc Block 5894
#[binrw]
#[derive(Debug, Clone)]
pub struct GPSUtc {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub reserved: u8,
    pub a_1: f32,
    pub a_0: f64,
    pub t_ot: u32,
    pub wn_t: u8,
    pub del_t_ls: i8,
    pub wn_lsf: u8,
    pub dn: u8,
    pub del_t_lsf: i8,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = write_vec)]
    pub padding: Vec<u8>,
}