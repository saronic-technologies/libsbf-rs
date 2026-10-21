use super::pvt_geodetic::{PvtError, PvtMode, PvtModeFlags};
use crate::util::binrw::{map_f4, map_u2, map_u4, unmap_f4, unmap_u2, unmap_u4};
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
    mode_raw: u8,
    error_raw: u8,
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

impl PosCovCartesian {
    /// PVT mode (bits 0-3 of mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.mode_raw)
    }

    /// Mode flags (bits 6-7 of mode).
    pub fn mode_flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.mode_raw)
    }

    /// PVT error code.
    pub fn error(&self) -> PvtError {
        PvtError::from(self.error_raw)
    }
}
