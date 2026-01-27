use binrw::BinRead;
use core::fmt;
use super::pvt_geodetic::{PvtMode, Datum};
use super::att_euler::AttitudeMode;

/// Combined GNSS mode containing PVT mode (bits 0-3) and attitude mode (bits 4-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GnssMode {
    raw: u8,
    pub pvt_mode: PvtMode,
    pub attitude_mode: AttitudeMode,
}

impl GnssMode {
    /// Parse from raw gnss_mode byte.
    pub fn from_byte(value: u8) -> Self {
        Self {
            raw: value,
            pvt_mode: PvtMode::from(value),
            attitude_mode: AttitudeMode::from((value >> 4) as u16),
        }
    }

    /// Get the raw byte value.
    pub fn raw(&self) -> u8 {
        self.raw
    }
}

impl fmt::Display for GnssMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PVT: {}, Attitude: {}", self.pvt_mode, self.attitude_mode)
    }
}

/// INS coupling mode (bits 0-2 of info field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum INSCouplingMode {
    #[default]
    LooselyCoupled = 0,
    Unknown,
}

impl From<u16> for INSCouplingMode {
    fn from(value: u16) -> Self {
        match value & 0x07 {
            0 => INSCouplingMode::LooselyCoupled,
            _ => INSCouplingMode::Unknown,
        }
    }
}

/// Solution output location (bits 3-5 of info field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum INSSolutionLocation {
    #[default]
    MainGnssAntenna = 0,
    FirstPoi = 1,
    Unknown,
}

impl From<u16> for INSSolutionLocation {
    fn from(value: u16) -> Self {
        match (value >> 3) & 0x07 {
            0 => INSSolutionLocation::MainGnssAntenna,
            1 => INSSolutionLocation::FirstPoi,
            _ => INSSolutionLocation::Unknown,
        }
    }
}

/// INS error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum INSError {
    #[default]
    None = 0,
    /// Position output prohibited due to export laws.
    PositionProhibited = 7,
    /// INS solution not requested by user.
    NotRequested = 20,
    /// Not enough valid external sensor measurements.
    NotEnoughSensorMeasurements = 21,
    /// Static alignment ongoing.
    StaticAlignmentOngoing = 23,
    /// Waiting for GNSS PVT.
    WaitingForGnssPvt = 24,
    /// In-motion alignment ongoing.
    InMotionAlignmentOngoing = 28,
    /// Waiting for GNSS heading.
    WaitingForGnssHeading = 29,
    /// Waiting for IMU to synchronize with PPS.
    WaitingForImuSync = 30,
    /// Standard deviation exceeds user limit (setINSStdDevMask).
    StdDevExceedsLimit = 31,
    /// Unsupported settings in INS.
    UnsupportedSettings = 32,
    /// Incorrect IMU orientation.
    IncorrectImuOrientation = 35,
}

impl From<u8> for INSError {
    fn from(value: u8) -> Self {
        match value {
            0 => INSError::None,
            7 => INSError::PositionProhibited,
            20 => INSError::NotRequested,
            21 => INSError::NotEnoughSensorMeasurements,
            23 => INSError::StaticAlignmentOngoing,
            24 => INSError::WaitingForGnssPvt,
            28 => INSError::InMotionAlignmentOngoing,
            29 => INSError::WaitingForGnssHeading,
            30 => INSError::WaitingForImuSync,
            31 => INSError::StdDevExceedsLimit,
            32 => INSError::UnsupportedSettings,
            35 => INSError::IncorrectImuOrientation,
            _ => INSError::None,
        }
    }
}

// INS Nav Geod Block 4226
#[derive(Debug, BinRead)]
pub struct INSNavGeod {
    #[br(map = |x| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    /// Bits 0-3: PVT mode, Bits 4-7: Attitude mode
    gnss_mode_raw: u8,
    pub error: u8,
    /// Bits 0-2: coupling mode, Bits 3-5: solution location, Bits 6-8: flags
    pub info: u16,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub gnss_age: Option<u16>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub latitude: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub longitude: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub height: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub undulation: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub accuracy: Option<u16>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub latency: Option<u16>,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(Datum::from(x)) })]
    pub datum: Option<Datum>,
    _reserved: u8,
    // TODO: unpack into an SBList type so we know what INSNav Sub Blocks we can parse
    pub sb_list: u16,

    #[br(if(sb_list & 1 == 1))]
    pub pos_std_dev: Option<INSNavGeodPosStdDev>,
    #[br(if((sb_list >> 1) & 1 == 1))]
    pub att: Option<INSNavGeodAtt>,
    #[br(if((sb_list >> 2) & 1 == 1))]
    pub att_std_dev: Option<INSNavGeodAttStdDev>,
    #[br(if((sb_list >> 3) & 1 == 1))]
    pub vel: Option<INSNavGeodVel>,
    #[br(if((sb_list >> 4) & 1 == 1))]
    pub vel_std_dev: Option<INSNavGeodVelStdDev>,
    #[br(if((sb_list >> 5) & 1 == 1))]
    pub pos_cov: Option<INSNavGeodPosCov>,
    #[br(if((sb_list >> 6) & 1 == 1))]
    pub att_cov: Option<INSNavGeodAttCov>,
    #[br(if((sb_list >> 7) & 1 == 1))]
    pub vel_cov: Option<INSNavGeodVelCov>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodPosStdDev {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub longitude_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub latitude_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub height_std_dev: Option<f32>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodAtt {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll: Option<f32>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodAttStdDev {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll_std_dev: Option<f32>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodVel {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vu: Option<f32>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodVelStdDev {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vu_std_dev: Option<f32>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodPosCov {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub latitude_longitude_cov: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub latitude_height_cov: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub longitude_height_cov: Option<f32>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodVelCov {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve_vn_cov: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve_vu_cov: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn_vu_cov: Option<f32>,
}

#[derive(Debug, BinRead)]
pub struct INSNavGeodAttCov {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_pitch_cov: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_roll_cov: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch_roll_cov: Option<f32>,
}

impl INSNavGeod {
    /// Combined GNSS mode (PVT mode + attitude mode).
    pub fn gnss_mode(&self) -> GnssMode {
        GnssMode::from_byte(self.gnss_mode_raw)
    }

    /// Last GNSS PVT mode used by INS filter (bits 0-3 of gnss_mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.gnss_mode_raw & 0x0F)
    }

    /// Last GNSS Attitude mode used by INS filter (bits 4-7 of gnss_mode).
    pub fn attitude_mode(&self) -> AttitudeMode {
        AttitudeMode::from((self.gnss_mode_raw >> 4) as u16)
    }

    /// INS error code.
    pub fn ins_error(&self) -> INSError {
        INSError::from(self.error)
    }

    /// INS coupling mode (bits 0-2 of info).
    pub fn coupling_mode(&self) -> INSCouplingMode {
        INSCouplingMode::from(self.info)
    }

    /// Solution output location (bits 3-5 of info).
    pub fn solution_location(&self) -> INSSolutionLocation {
        INSSolutionLocation::from(self.info)
    }

    /// Bit 6: 180-degree heading ambiguity fixed.
    pub fn heading_ambiguity_fixed(&self) -> bool {
        self.info & (1 << 6) != 0
    }

    /// Bit 7: Zero-velocity constraints used.
    pub fn zero_velocity_constraints(&self) -> bool {
        self.info & (1 << 7) != 0
    }

    /// Bit 8: IMU orientation estimation has converged.
    pub fn imu_orientation_converged(&self) -> bool {
        self.info & (1 << 8) != 0
    }
}
