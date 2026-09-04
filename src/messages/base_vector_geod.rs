use super::pvt_geodetic::{PvtError, PvtMode, PvtModeFlags};
use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// BaseVectorGeod Block 4028
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct BaseVectorGeod {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = binrw_util::unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = binrw_util::wrap_subblocks)]
    pub vectors: Vec<VectorInfoGeod>,
}

// VectorInfoGeod sub-block
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct VectorInfoGeod {
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub nr_sv: Option<u8>,
    #[br(map = |x: u8| PvtError::from(x))]
    #[bw(map = |x: &PvtError| u8::from(*x))]
    pub error: PvtError,
    mode_raw: u8,
    pub misc: u8,
    /// East component of the baseline in meters.
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub delta_east: Option<f64>,
    /// North component of the baseline in meters.
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub delta_north: Option<f64>,
    /// Up component of the baseline in meters.
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub delta_up: Option<f64>,
    /// East velocity relative to the base in meters per second.
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub delta_ve: Option<f32>,
    /// North velocity relative to the base in meters per second.
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub delta_vn: Option<f32>,
    /// Up velocity relative to the base in meters per second.
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub delta_vu: Option<f32>,
    /// Azimuth of the baseline in 0.01 degrees, 0 is North.
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub azimuth: Option<u16>,
    /// Elevation of the baseline in 0.01 degrees.
    #[br(map = binrw_util::map_i2)]
    #[bw(map = binrw_util::unmap_i2)]
    pub elevation: Option<i16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub reference_id: Option<u16>,
    /// Age of the differential corrections in 0.01 seconds.
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
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
