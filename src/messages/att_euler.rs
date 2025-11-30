use binrw::binrw;
use crate::do_not_use::{map_u1, map_u2, map_u4, map_f4, unmap_u1, unmap_u2, unmap_u4, unmap_f4};

// Attitude Euler Block 5938
#[binrw]
#[derive(Debug, Clone)]
pub struct AttEuler {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nrsv: Option<u8>,
    // TODO: create Error enum
    pub error: u8,
    pub mode: u16,
    _reserved: u16,

    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub roll: Option<f32>,

    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch_dot: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub roll_dot: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading_dot: Option<f32>,
}