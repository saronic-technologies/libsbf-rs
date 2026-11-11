use super::common::{PvtError, RawPvtMode};
use crate::util::binrw::{
    map_f4, map_u1_enum, map_u2, map_u4, unmap_f4, unmap_u1_enum, unmap_u2, unmap_u4,
};
use binrw::binrw;

// VelCovGeodetic Block 5908
#[binrw]
#[derive(Clone, Debug)]
pub struct VelCovGeodetic {
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
    pub cov_vnvn: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_veve: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vuvu: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_dd: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vnve: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vnvu: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vnd: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vevu: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_ved: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vud: Option<f32>,
}
