use super::common::GalMessageType;
use crate::util::binrw::{
    map_f4, map_f8, map_u1_enum, map_u2, map_u4, unmap_f4, unmap_f8, unmap_u1_enum, unmap_u2,
    unmap_u4,
};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// GALUtc Block 4031
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct GALUtc {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub svid: u8,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub source: GalMessageType,
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
    pub padding: Vec<u8>,
}
