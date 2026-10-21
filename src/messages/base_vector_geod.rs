use super::pvt_geodetic::{PvtError, PvtMisc, PvtMode, PvtModeFlags};
use crate::util::binrw::{
    map_f4, map_f8, map_i2, map_u1, map_u1_enum, map_u2, map_u4, map_u4_zero, unmap_f4, unmap_f8,
    unmap_i2, unmap_u1, unmap_u1_enum, unmap_u2, unmap_u4, unmap_u4_zero, unwrap_subblocks,
    wrap_subblocks,
};
use alloc::vec::Vec;
use binrw::binrw;

// BaseVectorGeod Block 4028
#[binrw]
#[derive(Clone, Debug)]
pub struct BaseVectorGeod {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Number of baselines in this block, one per base station.
    pub n: u8,
    /// Length of one sub-block in bytes.
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = wrap_subblocks)]
    /// Baseline from the rover to each base station.
    pub vectors: Vec<VectorInfoGeod>,
}

// VectorInfoGeod sub-block
#[binrw]
#[derive(Clone, Debug)]
pub struct VectorInfoGeod {
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    /// Number of satellites for which the PVT engine used corrections from this base station.
    pub nr_sv: Option<u8>,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    /// PVT error code for this baseline.
    pub error: PvtError,
    mode_raw: u8,
    #[br(map = PvtMisc::from_bits_retain)]
    #[bw(map = |x: &PvtMisc| x.bits())]
    /// Flags for whether the baseline runs from rover ARP to base ARP. The baseline is
    /// ARP-to-ARP only when bits 0 and 1 are both set. Bits 6-7 are reserved here.
    pub misc: PvtMisc,
    /// East component of the baseline from rover to base, in meters.
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_east: Option<f64>,
    /// North component of the baseline from rover to base, in meters.
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_north: Option<f64>,
    /// Up component of the baseline from rover to base, in meters.
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_up: Option<f64>,
    /// East velocity of the base with respect to the rover, in meters per second.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_ve: Option<f32>,
    /// North velocity of the base with respect to the rover, in meters per second.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_vn: Option<f32>,
    /// Up velocity of the base with respect to the rover, in meters per second.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_vu: Option<f32>,
    /// Azimuth of the base station in 0.01 degrees, from 0 to 360 increasing towards east.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub azimuth: Option<u16>,
    /// Elevation of the base station in 0.01 degrees, from -90 to 90.
    #[br(map = map_i2)]
    #[bw(map = unmap_i2)]
    pub elevation: Option<i16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    /// Base station ID.
    pub reference_id: Option<u16>,
    /// Age of the oldest differential correction used for this baseline, in 0.01 seconds.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub corr_age: Option<u16>,
    /// Signals with corrections available from this base station. Bit i is set for the
    /// signal with index i, and bit 0 is GPS L1 C/A.
    #[br(map = map_u4_zero)]
    #[bw(map = unmap_u4_zero)]
    pub signal_info: Option<u32>,
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
}
