use super::common::{Datum, PvtError, PvtMisc, RawAlertFlag, RawPvtMode, RawWaCorrInfo};
use crate::util::binrw::{
    map_f4, map_f8, map_u1, map_u1_enum, map_u1_enum_opt, map_u2, map_u2_zero, map_u4, map_u4_zero,
    unmap_f4, unmap_f8, unmap_u1, unmap_u1_enum, unmap_u1_enum_opt, unmap_u2, unmap_u2_zero,
    unmap_u4, unmap_u4_zero,
};
use alloc::vec::Vec;
use binrw::binrw;

// PosCart Block 4044
#[binrw]
#[derive(Clone, Debug)]
pub struct PosCart {
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
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub base2rover_x: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub base2rover_y: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub base2rover_z: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_xx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_yy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_zz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_xy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_xz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_yz: Option<f32>,
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub pdop: Option<u16>,
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub hdop: Option<u16>,
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub vdop: Option<u16>,
    #[br(map = PvtMisc::from_bits_retain)]
    #[bw(map = |x: &PvtMisc| x.bits())]
    pub misc: PvtMisc,
    _reserved: u8,
    /// Bit field indicating integrity related information.
    pub alert_flag: RawAlertFlag,
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
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
