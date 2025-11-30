use binrw::binrw;
use crate::do_not_use::{map_u2, map_u4, unmap_u2, unmap_u4};

// GEONav Block 5896
#[binrw]
#[derive(Debug, Clone)]
pub struct GEONav {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub reserved: u8,
    pub iodn: u16,
    pub ura: u16,
    pub t0: u32,
    pub xg: f64,
    pub yg: f64,
    pub zg: f64,
    pub xgd: f64,
    pub ygd: f64,
    pub zgd: f64,
    pub xgdd: f64,
    pub ygdd: f64,
    pub zgdd: f64,
    pub a_gf0: f32,
    pub a_gf1: f32,
}