use super::{
    att_euler::{AttitudeMode, BaselineError},
    pvt_geodetic::{
        Datum, DiffCorrType, PppInfo, PvtError, PvtMisc, PvtMode, PvtModeFlags, RaimIntegrity,
        TimeSystem, WACorrFlags,
    },
};
use crate::util::binrw::{
    map_f4, map_f8, map_i1, map_u1, map_u1_enum, map_u1_enum_opt, map_u1_zero, map_u2, map_u2_zero,
    map_u4, map_u64_zero, unmap_f4, unmap_f8, unmap_i1, unmap_u1, unmap_u1_enum, unmap_u1_enum_opt,
    unmap_u1_zero, unmap_u2, unmap_u2_zero, unmap_u4, unmap_u64_zero,
};
use alloc::vec::Vec;
use binrw::binrw;

// NavCart Block 4272
// Combined PVTCartesian + AttEuler + DOP + ReceiverTime fields.
#[binrw]
#[derive(Clone, Debug)]
pub struct NavCart {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    mode_raw: u8,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub error: PvtError,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub x: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub y: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub z: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub undulation: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cog: Option<f32>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub rx_clk_bias: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub rx_clk_drift: Option<f32>,
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub time_system: Option<TimeSystem>,
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub datum: Option<Datum>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nr_sv: Option<u8>,
    wa_corr_info_raw: u8,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub reference_id: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub mean_corr_age: Option<u16>,
    /// 64-bit signal tracking info (extended from PVTCartesian's 32-bit field).
    #[br(map = map_u64_zero)]
    #[bw(map = unmap_u64_zero)]
    pub signal_info: Option<u64>,
    alert_flag_raw: u8,
    /// Number of base stations used in the PVT computation.
    #[br(map = map_u1_zero)]
    #[bw(map = unmap_u1_zero)]
    pub nr_bases: Option<u8>,
    /// Age and type of the last PPP seed.
    pub ppp_info: PppInfo,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub latency: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub pos_h_acc: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub pos_v_acc: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub vel_h_acc: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub vel_v_acc: Option<u16>,
    #[br(map = PvtMisc::from_bits_retain)]
    #[bw(map = |x: &PvtMisc| x.bits())]
    pub misc: PvtMisc,
    _reserved: u8,
    // Attitude fields (from AttEuler)
    #[br(map = |x: u16| AttitudeMode::from(x))]
    #[bw(map = |x: &AttitudeMode| u16::from(*x))]
    pub attitude_mode: AttitudeMode,
    error_att_raw: u8,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nr_sv_att: Option<u8>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub roll: Option<f32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub heading_acc: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub pitch_acc: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub roll_acc: Option<u16>,
    // DOP field
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub pdop: Option<u16>,
    // UTC time fields
    #[br(map = map_i1)]
    #[bw(map = unmap_i1)]
    pub utc_hour: Option<i8>,
    #[br(map = map_i1)]
    #[bw(map = unmap_i1)]
    pub utc_min: Option<i8>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub utc_msec: Option<u16>,
    #[br(map = map_i1)]
    #[bw(map = unmap_i1)]
    pub utc_year: Option<i8>,
    #[br(map = map_i1)]
    #[bw(map = unmap_i1)]
    pub utc_month: Option<i8>,
    #[br(map = map_i1)]
    #[bw(map = unmap_i1)]
    pub utc_day: Option<i8>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl NavCart {
    // -- PVT accessors --

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

    /// Bit 5: GNSS signals may not be authentic, as flagged by SIG_AUTH_ALERT in RFStatus.
    pub fn signal_auth_alert(&self) -> bool {
        self.alert_flag_raw & (1 << 5) != 0
    }

    /// Bit 6: A navigation message failed authentication, as flagged by NAV_MSG_AUTH_ALERT
    /// in RFStatus.
    pub fn nav_msg_auth_alert(&self) -> bool {
        self.alert_flag_raw & (1 << 6) != 0
    }

    // -- Attitude accessors --

    /// Error code for Main-Aux1 baseline (bits 0-1 of error_att).
    pub fn main_aux1_error(&self) -> BaselineError {
        BaselineError::from(self.error_att_raw & 0x03)
    }

    /// Error code for Main-Aux2 baseline (bits 2-3 of error_att).
    pub fn main_aux2_error(&self) -> BaselineError {
        BaselineError::from((self.error_att_raw >> 2) & 0x03)
    }

    /// Bit 7 of error_att: attitude not requested by user.
    pub fn attitude_not_requested(&self) -> bool {
        self.error_att_raw & (1 << 7) != 0
    }
}
