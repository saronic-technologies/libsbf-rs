use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use bitflags::bitflags;

bitflags! {
    /// Flags bit field of the [`GPSCNav`] block.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GPSCNavFlags: u8 {
        /// Bit 0: at least one included message type had its alert bit set.
        const ALERT = 1 << 0;
        /// Bit 1: integrity status flag from MT10.
        const INTEGRITY_STATUS = 1 << 1;
        /// Bit 2: L2C phasing flag from MT10.
        const L2C_PHASING = 1 << 2;
        /// Bit 6: at least part of the navigation data was decoded from L2C.
        const L2C_USED = 1 << 6;
        /// Bit 7: at least part of the navigation data was decoded from L5.
        const L5_USED = 1 << 7;
    }
}

// GPSCNav Block 4042
#[binrw]
#[derive(Clone, Debug)]
pub struct GPSCNav {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub prn: u8,
    #[br(map = |x: u8| GPSCNavFlags::from_bits_retain(x))]
    #[bw(map = |x: &GPSCNavFlags| x.bits())]
    pub flags: GPSCNavFlags,
    pub wn: u16,
    pub health: u8,
    pub ura_ed: i8,
    pub t_op: u32,
    pub t_oe: u32,
    pub a: f64,
    pub a_dot: f64,
    pub delta_n: f32,
    pub delta_n_dot: f32,
    pub m_0: f64,
    pub e: f64,
    pub omega: f64,
    pub omega_0: f64,
    pub omegadot: f64,
    pub i_0: f64,
    pub idot: f32,
    pub c_is: f32,
    pub c_ic: f32,
    pub c_rs: f32,
    pub c_rc: f32,
    pub c_us: f32,
    pub c_uc: f32,
    pub t_oc: u32,
    pub ura_ned0: i8,
    pub ura_ned1: u8,
    pub ura_ned2: u8,
    pub wn_op: u8,
    pub a_f2: f32,
    pub a_f1: f32,
    pub a_f0: f64,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub t_gd: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub isc_l1ca: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub isc_l2c: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub isc_l5i5: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub isc_l5q5: Option<f32>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
