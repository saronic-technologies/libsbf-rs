use binrw::binrw;

use super::att_euler::BaselineError;

// AttCovEuler Block 5939
#[binrw]
#[derive(Debug)]
pub struct AttCovEuler {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub reserved: u8,
    error_raw: u8,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_head_head: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_pitch_pitch: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_roll_roll: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_head_pitch: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_head_roll: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_pitch_roll: Option<f32>,
}

impl AttCovEuler {
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
