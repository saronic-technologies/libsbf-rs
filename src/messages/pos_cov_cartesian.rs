use super::common::{PvtError, RawPvtMode};
use crate::util::binrw::{
    map_f4, map_u1_enum, map_u2, map_u4, unmap_f4, unmap_u1_enum, unmap_u2, unmap_u4,
};
use binrw::binrw;

// PosCovCartesian Block 5905
#[binrw]
#[derive(Clone, Debug)]
pub struct PosCovCartesian {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Bit field indicating the GNSS PVT mode.
    pub mode: RawPvtMode,
    /// PVT error code.
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub error: PvtError,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_xx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_yy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_zz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_bb: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_xy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_xz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_xb: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_yz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_yb: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_zb: Option<f32>,
}
