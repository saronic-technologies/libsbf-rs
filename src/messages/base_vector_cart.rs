use crate::SubBlock;
use alloc::vec::Vec;
use binrw::binrw;
use super::pvt_geodetic::{PvtError, PvtMode, PvtModeFlags};

// BaseVectorCart Block 4043
#[binrw]
#[derive(Clone, Debug)]
pub struct BaseVectorCart {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(crate::DO_NOT_USE_U4))]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) },
         map = |v: Vec<SubBlock<VectorInfoCart>>| v.into_iter().map(SubBlock::into_inner).collect())]
    #[bw(args_raw = (usize::from(*sb_length),),
         map = |v: &Vec<VectorInfoCart>| v.iter().cloned().map(SubBlock::from).collect::<Vec<_>>())]
    pub vectors: Vec<VectorInfoCart>,
}

// VectorInfoCart sub-block
#[binrw]
#[derive(Clone, Debug)]
pub struct VectorInfoCart {
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u8>| x.unwrap_or(crate::DO_NOT_USE_U1))]
    pub nr_sv: Option<u8>,
    #[br(map = |x: u8| PvtError::from(x))]
    #[bw(map = |x: &PvtError| u8::from(*x))]
    pub error: PvtError,
    mode_raw: u8,
    pub misc: u8,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub delta_x: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub delta_y: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub delta_z: Option<f64>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub delta_vx: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub delta_vy: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub delta_vz: Option<f32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub azimuth: Option<u16>,
    #[br(map = |x: i16| if x == crate::DO_NOT_USE_I2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<i16>| x.unwrap_or(crate::DO_NOT_USE_I2))]
    pub elevation: Option<i16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub reference_id: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub corr_age: Option<u16>,
    pub signal_info: u32,
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

    /// Bit 0: Baseline points to base station ARP.
    pub fn baseline_points_to_arp(&self) -> bool {
        self.misc & 1 != 0
    }

    /// Bit 1: Phase center offset compensated at rover.
    pub fn phase_center_compensated(&self) -> bool {
        self.misc & (1 << 1) != 0
    }
}
