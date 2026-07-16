use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use bitflags::bitflags;
use core::fmt;
use num_enum::{FromPrimitive, IntoPrimitive};

/// PVT mode (bits 0-3 of mode field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum PvtMode {
    NoPvt = 0,
    StandAlone = 1,
    Differential = 2,
    FixedLocation = 3,
    RtkFixed = 4,
    RtkFloat = 5,
    Sbas = 6,
    MovingBaseRtkFixed = 7,
    MovingBaseRtkFloat = 8,
    Ppp = 10,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl From<u8> for PvtMode {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0 => PvtMode::NoPvt,
            1 => PvtMode::StandAlone,
            2 => PvtMode::Differential,
            3 => PvtMode::FixedLocation,
            4 => PvtMode::RtkFixed,
            5 => PvtMode::RtkFloat,
            6 => PvtMode::Sbas,
            7 => PvtMode::MovingBaseRtkFixed,
            8 => PvtMode::MovingBaseRtkFloat,
            10 => PvtMode::Ppp,
            x => PvtMode::Unknown(x),
        }
    }
}

impl fmt::Display for PvtMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PvtMode::NoPvt => write!(f, "No PVT"),
            PvtMode::StandAlone => write!(f, "Stand Alone"),
            PvtMode::Differential => write!(f, "Differential"),
            PvtMode::FixedLocation => write!(f, "Fixed Location"),
            PvtMode::RtkFixed => write!(f, "RTK Fixed"),
            PvtMode::RtkFloat => write!(f, "RTK Float"),
            PvtMode::Sbas => write!(f, "SBAS"),
            PvtMode::MovingBaseRtkFixed => write!(f, "Moving Base RTK Fixed"),
            PvtMode::MovingBaseRtkFloat => write!(f, "Moving Base RTK Float"),
            PvtMode::Ppp => write!(f, "PPP"),
            PvtMode::Unknown(x) => write!(f, "Unknown({x})"),
        }
    }
}

bitflags! {
    /// Mode flags (bits 6-7 of mode field).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct PvtModeFlags: u8 {
        /// Bit 6: Receiver is determining static position.
        const DETERMINING_STATIC = 1 << 6;
        /// Bit 7: 2D mode (height assumed constant).
        const MODE_2D = 1 << 7;
    }
}

/// Coordinate datum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Datum {
    Wgs84 = 0,
    /// Datum equal to that used by the DGNSS/RTK base station.
    DgnssBaseStation = 19,
    /// ETRS89 (ETRF2000 realization).
    Etrs89 = 30,
    /// NAD83(2011), North American Datum (2011).
    Nad83_2011 = 31,
    /// NAD83(PA11), North American Datum, Pacific plate (2011).
    Nad83Pa11 = 32,
    /// NAD83(MA11), North American Datum, Marianas plate (2011).
    Nad83Ma11 = 33,
    /// GDA94(2010), Geocentric Datum of Australia (2010).
    Gda94 = 34,
    /// GDA2020, Geocentric Datum of Australia 2020.
    Gda2020 = 35,
    /// JGD2011, Japanese Geodetic Datum 2011.
    Jgd2011 = 36,
    /// First user-defined datum.
    UserDefined1 = 250,
    /// Second user-defined datum.
    UserDefined2 = 251,
    /// Unrecognized datum code.
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Datum {
    fn default() -> Self {
        Datum::Wgs84
    }
}

impl From<Datum> for u8 {
    fn from(value: Datum) -> Self {
        value as u8
    }
}

/// PVT error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum PvtError {
    None = 0,
    NotEnoughMeasurements = 1,
    NotEnoughEphemerides = 2,
    DopTooLarge = 3,
    ResidualsTooLarge = 4,
    NoConvergence = 5,
    NotEnoughAfterOutlierRejection = 6,
    PositionProhibited = 7,
    NotEnoughDiffCorrections = 8,
    BaseCoordsUnavailable = 9,
    AmbiguitiesNotFixed = 10,
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for PvtError {
    fn default() -> Self {
        PvtError::None
    }
}

impl From<PvtError> for u8 {
    fn from(value: PvtError) -> Self {
        value as u8
    }
}

bitflags! {
    /// Wide Area correction flags (bits 0-4 of wa_corr_info).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct WACorrFlags: u8 {
        /// Bit 0: Orbit and satellite clock correction used.
        const ORBIT_CLOCK = 1 << 0;
        /// Bit 1: Range correction used.
        const RANGE = 1 << 1;
        /// Bit 2: Ionospheric information used.
        const IONO = 1 << 2;
        /// Bit 3: Orbit accuracy (UERE/SISA) used.
        const ORBIT_ACCURACY = 1 << 3;
        /// Bit 4: DO229 Precision Approach mode active.
        const DO229_PRECISION_APPROACH = 1 << 4;
    }
}

/// Differential correction type (bits 5-6 of wa_corr_info).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum DiffCorrType {
    #[default]
    Unknown = 0,
    PhysicalBase = 1,
    VirtualBase = 2,
    Ssr = 3,
}

impl From<u8> for DiffCorrType {
    fn from(value: u8) -> Self {
        match (value >> 5) & 0x03 {
            0 => DiffCorrType::Unknown,
            1 => DiffCorrType::PhysicalBase,
            2 => DiffCorrType::VirtualBase,
            3 => DiffCorrType::Ssr,
            _ => DiffCorrType::Unknown,
        }
    }
}

/// RAIM integrity status (bits 0-1 of alert_flag).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum RaimIntegrity {
    #[default]
    NotActive = 0,
    TestSuccessful = 1,
    TestFailed = 2,
    Reserved = 3,
}

impl From<u8> for RaimIntegrity {
    fn from(value: u8) -> Self {
        match value & 0x03 {
            0 => RaimIntegrity::NotActive,
            1 => RaimIntegrity::TestSuccessful,
            2 => RaimIntegrity::TestFailed,
            3 => RaimIntegrity::Reserved,
            _ => RaimIntegrity::NotActive,
        }
    }
}

// PVTGeodetic Block 4007
#[binrw]
#[derive(Clone, Debug)]
pub struct PVTGeodetic {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    mode_raw: u8,
    #[br(map = |x: u8| PvtError::from(x))]
    #[bw(map = |x: &PvtError| u8::from(*x))]
    pub error: PvtError,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub latitude: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub longitude: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub height: Option<f64>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub undulation: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vn: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub ve: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vu: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub cog: Option<f32>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub rx_clk_bias: Option<f64>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub rx_clk_drift: Option<f32>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub time_system: Option<u8>,
    #[br(map = binrw_util::map_datum)]
    #[bw(map = binrw_util::unmap_datum)]
    pub datum: Option<Datum>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub nr_sv: Option<u8>,
    wa_corr_info_raw: u8,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub reference_id: Option<u16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub mean_corr_age: Option<u16>,
    pub signal_info: u32,
    alert_flag_raw: u8,
    // Rev 1 fields
    pub nr_bases: u8,
    pub ppp_info: u16,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub latency: Option<u16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub h_accuracy: Option<u16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub v_accuracy: Option<u16>,
    pub misc: u8,
    // Rev 2 fields
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl PVTGeodetic {
    /// PVT mode (bits 0-3 of mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.mode_raw)
    }

    /// Mode flags (bits 6-7 of mode).
    pub fn mode_flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.mode_raw)
    }

    /// Wide Area correction flags (bits 0-4).
    pub fn wa_corr_flags(&self) -> WACorrFlags {
        WACorrFlags::from_bits_truncate(self.wa_corr_info_raw)
    }

    /// Differential correction type (bits 5-6).
    pub fn diff_corr_type(&self) -> DiffCorrType {
        DiffCorrType::from(self.wa_corr_info_raw)
    }

    /// RAIM integrity status (bits 0-1 of alert_flag).
    pub fn raim_integrity(&self) -> RaimIntegrity {
        RaimIntegrity::from(self.alert_flag_raw)
    }

    /// Bit 2: Galileo HPCA integrity failed.
    pub fn galileo_hpca_failed(&self) -> bool {
        self.alert_flag_raw & (1 << 2) != 0
    }

    /// Bit 3: Galileo ionospheric storm active.
    pub fn galileo_iono_storm(&self) -> bool {
        self.alert_flag_raw & (1 << 3) != 0
    }
}