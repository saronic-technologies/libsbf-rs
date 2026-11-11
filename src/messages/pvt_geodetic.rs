use super::common::{
    Datum, PppInfo, PvtError, PvtMisc, RawAlertFlag, RawPvtMode, RawWaCorrInfo, TimeSystem,
};
use crate::util::binrw::{
    map_f4, map_f8, map_u1, map_u1_enum, map_u1_enum_opt, map_u1_zero, map_u2, map_u4, map_u4_zero,
    unmap_f4, unmap_f8, unmap_u1, unmap_u1_enum, unmap_u1_enum_opt, unmap_u1_zero, unmap_u2,
    unmap_u4, unmap_u4_zero,
};
use alloc::vec::Vec;
use binrw::binrw;

// PVTGeodetic Block 4007
#[binrw]
#[derive(Clone, Debug)]
pub struct PVTGeodetic {
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
    pub latitude: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub longitude: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub height: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub undulation: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vn: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ve: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vu: Option<f32>,
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
    #[br(map = map_u4_zero)]
    #[bw(map = unmap_u4_zero)]
    pub signal_info: Option<u32>,
    /// Bit field indicating integrity related information.
    pub alert_flag: RawAlertFlag,
    // Rev 1 fields
    #[br(map = map_u1_zero)]
    #[bw(map = unmap_u1_zero)]
    pub nr_bases: Option<u8>,
    pub ppp_info: PppInfo,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub latency: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub h_accuracy: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub v_accuracy: Option<u16>,
    #[br(map = PvtMisc::from_bits_retain)]
    #[bw(map = |x: &PvtMisc| x.bits())]
    pub misc: PvtMisc,
    // Rev 2 fields
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
