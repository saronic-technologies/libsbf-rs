use super::pvt_geodetic::{PvtError, PvtMisc, PvtMode, PvtModeFlags};
use crate::util::binrw::{
    map_f4, map_f8, map_i2, map_u1, map_u1_enum, map_u2, map_u4, map_u4_zero, unmap_f4, unmap_f8,
    unmap_i2, unmap_u1, unmap_u1_enum, unmap_u2, unmap_u4, unmap_u4_zero, unwrap_subblocks,
    wrap_subblocks,
};
use alloc::vec::Vec;
use binrw::binrw;

// BaseVectorCart Block 4043
#[binrw]
#[derive(Clone, Debug)]
pub struct BaseVectorCart {
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
    pub vectors: Vec<VectorInfoCart>,
}

// VectorInfoCart sub-block
#[binrw]
#[derive(Clone, Debug)]
pub struct VectorInfoCart {
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nr_sv: Option<u8>,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub error: PvtError,
    mode_raw: u8,
    #[br(map = PvtMisc::from_bits_retain)]
    #[bw(map = |x: &PvtMisc| x.bits())]
    pub misc: PvtMisc,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_x: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_y: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_z: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_vx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_vy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_vz: Option<f32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub azimuth: Option<u16>,
    #[br(map = map_i2)]
    #[bw(map = unmap_i2)]
    pub elevation: Option<i16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub reference_id: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub corr_age: Option<u16>,
    #[br(map = map_u4_zero)]
    #[bw(map = unmap_u4_zero)]
    pub signal_info: Option<u32>,
}

impl VectorInfoCart {
    /// PVT mode (bits 0-3 of mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.mode_raw)
    }

    /// Mode flags (bits 6-7 of mode).
    pub fn mode_flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.mode_raw)
    }
}
