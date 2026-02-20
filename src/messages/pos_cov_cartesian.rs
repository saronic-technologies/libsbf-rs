use binrw::binrw;

use super::pvt_geodetic::{PvtError, PvtMode, PvtModeFlags};

// PosCovCartesian Block 5905
#[binrw]
#[derive(Debug)]
pub struct PosCovCartesian {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    mode_raw: u8,
    error_raw: u8,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_xx: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_yy: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_zz: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_bb: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_xy: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_xz: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_xb: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_yz: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_yb: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
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
