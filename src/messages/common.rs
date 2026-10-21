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

/// The raw Mode byte of the PVT blocks, which decodes into the PVT mode and the mode flags.
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawPvtMode(u8);

impl RawPvtMode {
    /// PVT mode from bits 0-3.
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.0)
    }

    /// Mode flags from bits 6-7.
    pub fn flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.0)
    }
}

/// Coordinate datum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Datum {
    /// WGS84/ITRS.
    Wgs84 = 0,
    /// Datum equal to that used by the DGNSS/RTK base station.
    DgnssBaseStation = 19,
    /// Local datum defined by the RTCMDatum block with datum code 20.
    RtcmLocal20 = 20,
    /// Local datum defined by the RTCMDatum block with datum code 21.
    RtcmLocal21 = 21,
    /// Local datum defined by the RTCMDatum block with datum code 22.
    RtcmLocal22 = 22,
    /// Local datum defined by the RTCMDatum block with datum code 23.
    RtcmLocal23 = 23,
    /// Local datum defined by the RTCMDatum block with datum code 24.
    RtcmLocal24 = 24,
    /// Local coordinate reference system selected with setLocalCoordOperation.
    LocalCoordOperation = 25,
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

/// Time system that the receiver clock bias in a PVT block refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum TimeSystem {
    /// GPS time.
    Gps = 0,
    /// Galileo time.
    Galileo = 1,
    /// GLONASS time.
    Glonass = 3,
    /// BeiDou time.
    BeiDou = 4,
    /// QZSS time.
    Qzss = 5,
    /// Fugro AtomiChron time.
    Fugro = 100,
    /// Time system code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
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
    /// Reported only by PosLocal and PosProjected.
    DatumTransformationUnknown = 17,
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for PvtError {
    fn default() -> Self {
        PvtError::None
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

/// The raw WACorrInfo byte of the PVT blocks, which decodes into the wide area correction flags
/// and the differential correction type.
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawWaCorrInfo(u8);

impl RawWaCorrInfo {
    /// Wide Area correction flags from bits 0-4.
    pub fn wa_corr_flags(&self) -> WACorrFlags {
        WACorrFlags::from_bits_truncate(self.0)
    }

    /// Differential correction type from bits 5-6.
    pub fn diff_corr_type(&self) -> DiffCorrType {
        DiffCorrType::from(self.0)
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

/// The raw AlertFlag byte of the PVT blocks, which decodes into the RAIM integrity status and the
/// integrity and authentication alerts.
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawAlertFlag(u8);

impl RawAlertFlag {
    /// RAIM integrity status from bits 0-1.
    pub fn raim_integrity(&self) -> RaimIntegrity {
        RaimIntegrity::from(self.0)
    }

    /// Bit 2: Galileo HPCA integrity failed.
    pub fn galileo_hpca_failed(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Bit 3: Galileo ionospheric storm active.
    pub fn galileo_iono_storm(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Bit 5: GNSS signals may not be authentic, as flagged by SIG_AUTH_ALERT in RFStatus.
    pub fn signal_auth_alert(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Bit 6: A navigation message failed authentication, as flagged by NAV_MSG_AUTH_ALERT
    /// in RFStatus.
    pub fn nav_msg_auth_alert(&self) -> bool {
        self.0 & (1 << 6) != 0
    }
}

/// Type of the last PPP seed, from bits 13-15 of ppp_info.
#[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum PppSeedType {
    /// Not seeded or not in PPP positioning mode.
    NotSeeded = 0,
    Manual = 1,
    Dgnss = 2,
    RtkFixed = 3,
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Whether the marker position reported in a block is also the ARP position,
/// from bits 6-7 of misc.
#[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ArpOffset {
    Unknown = 0,
    /// The ARP-to-marker offset is zero.
    Zero = 1,
    /// The ARP-to-marker offset is not zero.
    NonZero = 2,
    #[num_enum(catch_all)]
    Reserved(u8),
}

/// PPPInfo bit field of the PVT blocks: the seed age in bits 0-11 and the
/// seed type in bits 13-15.
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PppInfo(u16);

impl PppInfo {
    const SEED_AGE_MASK: u16 = 0x0FFF;
    const SEED_TYPE_SHIFT: u32 = 13;

    /// Age of the last PPP seed in seconds, clipped to 4091. Ignore when the
    /// seed type is NotSeeded.
    pub fn seed_age(&self) -> u16 {
        self.0 & Self::SEED_AGE_MASK
    }

    /// Type of the last PPP seed.
    pub fn seed_type(&self) -> PppSeedType {
        PppSeedType::from((self.0 >> Self::SEED_TYPE_SHIFT) as u8)
    }
}

bitflags! {
    /// Misc bit field of the PVT and base vector blocks.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PvtMisc: u8 {
        /// Bit 0: in DGNSS or RTK mode, the baseline points to the base
        /// station ARP rather than the antenna phase center.
        const BASELINE_TO_BASE_ARP = 1 << 0;
        /// Bit 1: the phase center offset is compensated for at the rover.
        const PHASE_CENTER_COMPENSATED = 1 << 1;
        /// Mask for the ARP-to-marker offset flag in bits 6-7, decoded by
        /// [`Self::arp_offset`]. Reserved in the base vector blocks.
        const ARP_OFFSET_MASK = 0b11 << Self::ARP_OFFSET_SHIFT;
    }
}

impl PvtMisc {
    const ARP_OFFSET_SHIFT: u32 = 6;

    /// ARP-to-marker offset flag, from bits 6-7.
    pub fn arp_offset(&self) -> ArpOffset {
        ArpOffset::from(self.intersection(Self::ARP_OFFSET_MASK).bits() >> Self::ARP_OFFSET_SHIFT)
    }
}

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

/// Combined GNSS mode containing PVT mode (bits 0-3) and attitude mode (bits 4-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum INSCouplingMode {
    LooselyCoupled = 0,
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for INSCouplingMode {
    fn default() -> Self {
        INSCouplingMode::LooselyCoupled
    }
}

impl From<u16> for INSCouplingMode {
    fn from(value: u16) -> Self {
        match value & 0x07 {
            0 => INSCouplingMode::LooselyCoupled,
            x => INSCouplingMode::Unknown(x as u8),
        }
    }
}

/// Solution output location (bits 3-5 of info field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum INSSolutionLocation {
    MainGnssAntenna = 0,
    FirstPoi = 1,
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for INSSolutionLocation {
    fn default() -> Self {
        INSSolutionLocation::MainGnssAntenna
    }
}

impl From<u16> for INSSolutionLocation {
    fn from(value: u16) -> Self {
        match (value >> 3) & 0x07 {
            0 => INSSolutionLocation::MainGnssAntenna,
            1 => INSSolutionLocation::FirstPoi,
            x => INSSolutionLocation::Unknown(x as u8),
        }
    }
}

/// The raw Info field of the INS blocks, which decodes into the INS coupling mode, the solution
/// output location and the INS status flags.
#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawInsInfo(u16);

impl RawInsInfo {
    /// INS coupling mode from bits 0-2.
    pub fn coupling_mode(&self) -> INSCouplingMode {
        INSCouplingMode::from(self.0)
    }

    /// Solution output location from bits 3-5.
    pub fn solution_location(&self) -> INSSolutionLocation {
        INSSolutionLocation::from(self.0)
    }

    /// Bit 6: 180-degree heading ambiguity fixed.
    pub fn heading_ambiguity_fixed(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Bit 7: Zero-velocity constraints used.
    pub fn zero_velocity_constraints(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Bit 8: IMU orientation estimation has converged.
    pub fn imu_orientation_converged(&self) -> bool {
        self.0 & (1 << 8) != 0
    }
}

/// INS error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum INSError {
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
    /// Unrecognized error code.
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for INSError {
    fn default() -> Self {
        INSError::None
    }
}

/// Connection port / external sensor source.
///
/// Used by ExtSensorMeas, ExtSensorStatus, ExtSensorInfo, IMUSetup and VelSensorSetup to
/// identify the receiver port an external sensor is connected to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ConnectionPort {
    Com1 = 0,
    Com2 = 1,
    Com3 = 2,
    Com4 = 3,
    Gpio = 4,
    Usb1 = 5,
    Usb2 = 6,
    Ip10 = 7,
    Ip11 = 8,
    Ip12 = 9,
    Ip13 = 10,
    Ip14 = 11,
    Ip15 = 12,
    Ip16 = 13,
    Ip17 = 14,
    Ips1 = 15,
    Ips2 = 16,
    Ips3 = 17,
    Ips4 = 18,
    Ips5 = 19,
    Ipr1 = 20,
    Ipr2 = 21,
    Ipr3 = 22,
    Ipr4 = 23,
    Ipr5 = 24,
    InternalSpi = 32,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl fmt::Display for ConnectionPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionPort::Com1 => write!(f, "COM1"),
            ConnectionPort::Com2 => write!(f, "COM2"),
            ConnectionPort::Com3 => write!(f, "COM3"),
            ConnectionPort::Com4 => write!(f, "COM4"),
            ConnectionPort::Gpio => write!(f, "GPIO"),
            ConnectionPort::Usb1 => write!(f, "USB1"),
            ConnectionPort::Usb2 => write!(f, "USB2"),
            ConnectionPort::Ip10 => write!(f, "IP10"),
            ConnectionPort::Ip11 => write!(f, "IP11"),
            ConnectionPort::Ip12 => write!(f, "IP12"),
            ConnectionPort::Ip13 => write!(f, "IP13"),
            ConnectionPort::Ip14 => write!(f, "IP14"),
            ConnectionPort::Ip15 => write!(f, "IP15"),
            ConnectionPort::Ip16 => write!(f, "IP16"),
            ConnectionPort::Ip17 => write!(f, "IP17"),
            ConnectionPort::Ips1 => write!(f, "IPS1"),
            ConnectionPort::Ips2 => write!(f, "IPS2"),
            ConnectionPort::Ips3 => write!(f, "IPS3"),
            ConnectionPort::Ips4 => write!(f, "IPS4"),
            ConnectionPort::Ips5 => write!(f, "IPS5"),
            ConnectionPort::Ipr1 => write!(f, "IPR1"),
            ConnectionPort::Ipr2 => write!(f, "IPR2"),
            ConnectionPort::Ipr3 => write!(f, "IPR3"),
            ConnectionPort::Ipr4 => write!(f, "IPR4"),
            ConnectionPort::Ipr5 => write!(f, "IPR5"),
            ConnectionPort::InternalSpi => write!(f, "Internal SPI"),
            ConnectionPort::Unknown(x) => write!(f, "Unknown: {x}"),
        }
    }
}

/// External sensor model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ExtSensorModel {
    /// Ellipse from SBG Systems.
    SbgEllipse = 2,
    /// Ellipse2 from SBG Systems.
    SbgEllipse2 = 5,
    /// VN-100 from VectorNav.
    Vn100 = 7,
    /// ADIS1650x from Analog Devices.
    Adis1650x = 10,
    /// Zero-velocity sensor.
    ZeroVelocity = 20,
    /// Velocity input.
    VelocityInput = 21,
    /// Sensor model code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for ExtSensorModel {
    fn default() -> Self {
        ExtSensorModel::Unknown(0)
    }
}

impl fmt::Display for ExtSensorModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtSensorModel::SbgEllipse => write!(f, "SBG Ellipse"),
            ExtSensorModel::SbgEllipse2 => write!(f, "SBG Ellipse 2"),
            ExtSensorModel::Vn100 => write!(f, "VN-100"),
            ExtSensorModel::Adis1650x => write!(f, "ADIS1650x"),
            ExtSensorModel::ZeroVelocity => write!(f, "Zero Velocity"),
            ExtSensorModel::VelocityInput => write!(f, "Velocity Input"),
            ExtSensorModel::Unknown(x) => write!(f, "Unknown: {x}"),
        }
    }
}

/// The Galileo navigation message type from which a block's data has been decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum GalMessageType {
    /// I/NAV, which carries the (L1,E5b) clock model.
    INav = 2,
    /// F/NAV, which carries the (L1,E5a) clock model.
    FNav = 16,
    /// Message type code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Rise/set state of a satellite.
#[derive(Clone, Copy, Debug)]
pub enum RiseSet {
    Setting,
    Rising,
    Unknown,
    Other(u8),
}

impl From<u8> for RiseSet {
    fn from(value: u8) -> Self {
        match value {
            0 => RiseSet::Setting,
            1 => RiseSet::Rising,
            // 3 in the 2-bit ChannelStatus field, 255 in SatVisibility.
            3 | 255 => RiseSet::Unknown,
            other => RiseSet::Other(other),
        }
    }
}

impl From<RiseSet> for u8 {
    fn from(value: RiseSet) -> Self {
        match value {
            RiseSet::Setting => 0,
            RiseSet::Rising => 1,
            RiseSet::Unknown => 255,
            RiseSet::Other(other) => other,
        }
    }
}
