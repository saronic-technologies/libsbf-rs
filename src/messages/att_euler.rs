use crate::binrw_util;
use binrw::binrw;
use core::fmt;
use num_enum::{FromPrimitive, IntoPrimitive};

/// Attitude mode code.
///
/// INS blocks use codes 0, 5-6 in firmware versions after 1.5.0
/// instead of copying 0-4 from the attitude.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum AttitudeMode {
    NoAttitude = 0,
    /// Heading, pitch (roll=0), aux antenna positions with float ambiguities.
    HeadingPitchFloat = 1,
    /// Heading, pitch (roll=0), aux antenna positions with fixed ambiguities.
    HeadingPitchFixed = 2,
    /// Heading, pitch, roll, aux antenna positions with float ambiguities.
    HeadingPitchRollFloat = 3,
    /// Heading, pitch, roll, aux antenna positions with fixed ambiguities.
    HeadingPitchRollFixed = 4,
    /// INS attitude is not fused with GNSS attitude.
    InsNotFused = 5,
    /// INS attitude is fused with GNSS attitude.
    InsFused = 6,
    /// Unrecognized attitude mode code.
    #[num_enum(catch_all)]
    Unknown(u16),
}

#[allow(clippy::derivable_impls)]
impl Default for AttitudeMode {
    fn default() -> Self {
        AttitudeMode::NoAttitude
    }
}

impl fmt::Display for AttitudeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttitudeMode::NoAttitude => write!(f, "No Attitude"),
            AttitudeMode::HeadingPitchFloat => write!(f, "Heading+Pitch Float"),
            AttitudeMode::HeadingPitchFixed => write!(f, "Heading+Pitch Fixed"),
            AttitudeMode::HeadingPitchRollFloat => write!(f, "Heading+Pitch+Roll Float"),
            AttitudeMode::HeadingPitchRollFixed => write!(f, "Heading+Pitch+Roll Fixed"),
            AttitudeMode::InsNotFused => write!(f, "INS Not Fused With GNSS Attitude"),
            AttitudeMode::InsFused => write!(f, "INS Fused With GNSS Attitude"),
            AttitudeMode::Unknown(x) => write!(f, "Unknown({x})"),
        }
    }
}

/// Baseline error code (2 bits per baseline).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum BaselineError {
    #[default]
    NoError = 0,
    NotEnoughMeasurements = 1,
    Reserved2 = 2,
    Reserved3 = 3,
}

impl From<u8> for BaselineError {
    fn from(value: u8) -> Self {
        match value & 0x03 {
            0 => BaselineError::NoError,
            1 => BaselineError::NotEnoughMeasurements,
            2 => BaselineError::Reserved2,
            3 => BaselineError::Reserved3,
            _ => BaselineError::NoError,
        }
    }
}

// Attitude Euler Block 5938
#[binrw]
#[derive(Clone, Debug)]
pub struct AttEuler {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub nrsv: Option<u8>,
    error_raw: u8,
    mode_raw: u16,
    _reserved: u16,

    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub heading: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub pitch: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub roll: Option<f32>,

    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub pitch_dot: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub roll_dot: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub heading_dot: Option<f32>,
}

impl AttEuler {
    /// Attitude mode.
    pub fn mode(&self) -> AttitudeMode {
        AttitudeMode::from(self.mode_raw)
    }

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
