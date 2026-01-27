use binrw::binrw;
use core::fmt;

/// Attitude mode code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum AttitudeMode {
    #[default]
    NoAttitude = 0,
    /// Heading, pitch (roll=0), aux antenna positions with float ambiguities.
    HeadingPitchFloat = 1,
    /// Heading, pitch (roll=0), aux antenna positions with fixed ambiguities.
    HeadingPitchFixed = 2,
    /// Heading, pitch, roll, aux antenna positions with float ambiguities.
    HeadingPitchRollFloat = 3,
    /// Heading, pitch, roll, aux antenna positions with fixed ambiguities.
    HeadingPitchRollFixed = 4,
}

impl From<u16> for AttitudeMode {
    fn from(value: u16) -> Self {
        match value & 0x0F {
            0 => AttitudeMode::NoAttitude,
            1 => AttitudeMode::HeadingPitchFloat,
            2 => AttitudeMode::HeadingPitchFixed,
            3 => AttitudeMode::HeadingPitchRollFloat,
            4 => AttitudeMode::HeadingPitchRollFixed,
            _ => AttitudeMode::NoAttitude,
        }
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
#[derive(Debug)]
pub struct AttEuler {
    #[br(map = |x| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub nrsv: Option<u8>,
    error_raw: u8,
    mode_raw: u16,
    _reserved: u16,

    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll: Option<f32>,

    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch_dot: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll_dot: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
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
