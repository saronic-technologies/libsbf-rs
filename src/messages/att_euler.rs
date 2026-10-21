use super::common::{AttitudeMode, BaselineError};
use crate::util::binrw::{map_f4, map_u1, map_u2, map_u4, unmap_f4, unmap_u1, unmap_u2, unmap_u4};
use binrw::binrw;

// Attitude Euler Block 5938
#[binrw]
#[derive(Clone, Debug)]
pub struct AttEuler {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nrsv: Option<u8>,
    error_raw: u8,
    #[br(map = |x: u16| AttitudeMode::from(x))]
    #[bw(map = |x: &AttitudeMode| u16::from(*x))]
    pub mode: AttitudeMode,
    _reserved: u16,

    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub roll: Option<f32>,

    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch_dot: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub roll_dot: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading_dot: Option<f32>,
}

impl AttEuler {
    /// Error code for Main-Aux1 baseline (bits 0-1).
    pub fn main_aux1_error(&self) -> BaselineError {
        BaselineError::from(self.error_raw & 0x03)
    }

    /// Error code for Main-Aux2 baseline (bits 2-3).
    pub fn main_aux2_error(&self) -> BaselineError {
        BaselineError::from((self.error_raw >> 2) & 0x03)
    }

    /// Bit 7: Returns true if attitude was not requested by user.
    pub fn not_requested(&self) -> bool {
        self.error_raw & (1 << 7) != 0
    }
}
