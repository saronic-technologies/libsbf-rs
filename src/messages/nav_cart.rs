use alloc::vec::Vec;
use binrw::BinRead;

use super::att_euler::{AttitudeMode, BaselineError};
use super::pvt_geodetic::{
    Datum, DiffCorrType, PvtError, PvtMode, PvtModeFlags, RaimIntegrity, WACorrFlags,
};

// NavCart Block 4272
// Combined PVTCartesian + AttEuler + DOP + ReceiverTime fields.
#[derive(Debug, BinRead)]
pub struct NavCart {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    mode_raw: u8,
    #[br(map = |x: u8| PvtError::from(x))]
    pub error: PvtError,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub x: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub y: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub z: Option<f64>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub undulation: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vx: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vy: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vz: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cog: Option<f32>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub rx_clk_bias: Option<f64>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub rx_clk_drift: Option<f32>,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub time_system: Option<u8>,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(Datum::from(x)) })]
    pub datum: Option<Datum>,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub nr_sv: Option<u8>,
    wa_corr_info_raw: u8,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub reference_id: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub mean_corr_age: Option<u16>,
    /// 64-bit signal tracking info (extended from PVTCartesian's 32-bit field).
    pub signal_info: u64,
    alert_flag_raw: u8,
    pub nr_bases: u8,
    pub ppp_info: u16,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub latency: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub pos_h_acc: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub pos_v_acc: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub vel_h_acc: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub vel_v_acc: Option<u16>,
    pub misc: u8,
    _reserved: u8,
    // Attitude fields (from AttEuler)
    mode_att_raw: u16,
    error_att_raw: u8,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub nr_sv_att: Option<u8>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll: Option<f32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub heading_acc: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub pitch_acc: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub roll_acc: Option<u16>,
    // DOP field
    #[br(map = |x: u16| if x == 0 { None } else { Some(x) })]
    pub pdop: Option<u16>,
    // UTC time fields
    #[br(map = |x: i8| if x == crate::DO_NOT_USE_I1 { None } else { Some(x) })]
    pub utc_hour: Option<i8>,
    #[br(map = |x: i8| if x == crate::DO_NOT_USE_I1 { None } else { Some(x) })]
    pub utc_min: Option<i8>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub utc_msec: Option<u16>,
    #[br(map = |x: i8| if x == crate::DO_NOT_USE_I1 { None } else { Some(x) })]
    pub utc_year: Option<i8>,
    #[br(map = |x: i8| if x == crate::DO_NOT_USE_I1 { None } else { Some(x) })]
    pub utc_month: Option<i8>,
    #[br(map = |x: i8| if x == crate::DO_NOT_USE_I1 { None } else { Some(x) })]
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

    // -- Attitude accessors --

    /// Attitude mode.
    pub fn attitude_mode(&self) -> AttitudeMode {
        AttitudeMode::from(self.mode_att_raw)
    }

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
