use alloc::vec::Vec;
use binrw::BinRead;

use super::pvt_geodetic::{PvtError, PvtMode, PvtModeFlags};

// BaseVectorGeod Block 4028
#[derive(BinRead, Clone, Debug)]
pub struct BaseVectorGeod {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(count = usize::from(n))]
    pub vectors: Vec<VectorInfoGeod>,
}

// VectorInfoGeod sub-block
#[derive(BinRead, Clone, Debug)]
pub struct VectorInfoGeod {
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub nr_sv: Option<u8>,
    #[br(map = |x: u8| PvtError::from(x))]
    pub error: PvtError,
    mode_raw: u8,
    pub misc: u8,
    /// East component of the baseline in meters.
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub delta_east: Option<f64>,
    /// North component of the baseline in meters.
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub delta_north: Option<f64>,
    /// Up component of the baseline in meters.
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub delta_up: Option<f64>,
    /// East velocity relative to the base in meters per second.
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_ve: Option<f32>,
    /// North velocity relative to the base in meters per second.
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_vn: Option<f32>,
    /// Up velocity relative to the base in meters per second.
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_vu: Option<f32>,
    /// Azimuth of the baseline in 0.01 degrees, 0 is North.
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub azimuth: Option<u16>,
    /// Elevation of the baseline in 0.01 degrees.
    #[br(map = |x: i16| if x == crate::DO_NOT_USE_I2 { None } else { Some(x) })]
    pub elevation: Option<i16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub reference_id: Option<u16>,
    /// Age of the differential corrections in 0.01 seconds.
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub corr_age: Option<u16>,
    pub signal_info: u32,
}

impl VectorInfoGeod {
    /// PVT mode from bits 0-3 of mode.
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.mode_raw)
    }

    /// Mode flags from bits 6-7 of mode.
    pub fn mode_flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.mode_raw)
    }

    /// Bit 0: baseline points to base station ARP.
    pub fn baseline_points_to_arp(&self) -> bool {
        self.misc & 1 != 0
    }

    /// Bit 1: phase center offset compensated at rover.
    pub fn phase_center_compensated(&self) -> bool {
        self.misc & (1 << 1) != 0
    }
}
