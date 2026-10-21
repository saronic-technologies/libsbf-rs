use super::common::{
    AttitudeMode, Datum, PppInfo, PvtError, PvtMisc, RawAlertFlag, RawAttitudeError, RawPvtMode,
    RawWaCorrInfo, TimeSystem,
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
    /// Bit field indicating the GNSS PVT mode.
    pub mode: RawPvtMode,
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
    /// Bit field providing information about which corrections have been applied.
    pub wa_corr_info: RawWaCorrInfo,
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
    /// Bit field indicating integrity related information.
    pub alert_flag: RawAlertFlag,
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
    /// Bit field providing error information.
    pub error_att: RawAttitudeError,
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
