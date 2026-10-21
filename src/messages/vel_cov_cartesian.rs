use super::common::{PvtError, PvtMode, PvtModeFlags};
use crate::util::binrw::{
    map_f4, map_u1_enum, map_u2, map_u4, unmap_f4, unmap_u1_enum, unmap_u2, unmap_u4,
};
use binrw::binrw;

// VelCovCartesian Block 5907
#[binrw]
#[derive(Clone, Debug)]
pub struct VelCovCartesian {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    mode_raw: u8,
    /// PVT error code.
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub error: PvtError,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vxvx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vyvy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vzvz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_dd: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vxvy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vxvz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vxd: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vyvz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vyd: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_vzd: Option<f32>,
}

impl VelCovCartesian {
    /// PVT mode (bits 0-3 of mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.mode_raw)
    }

    /// Mode flags (bits 6-7 of mode).
    pub fn mode_flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.mode_raw)
    }
}
