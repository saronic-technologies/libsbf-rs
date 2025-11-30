use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_u2, map_u4, unmap_u2, unmap_u4, write_vec};

// GPSNav Block 5891
#[binrw]
#[derive(Debug, Clone)]
pub struct GPSNav {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub reserved: u8,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wn: Option<u16>,
    pub ca_or_p_on_l2: u8,
    pub ura: u8,
    pub health: u8,
    pub l2_data_flag: u8,
    pub iodc: u16,
    pub iode2: u8,
    pub iode3: u8,
    pub fit_int_flg: u8,
    pub reserved2: u8,
    pub t_gd: f32,
    pub t_oc: u32,
    pub a_f2: f32,
    pub a_f1: f32,
    pub a_f0: f32,
    pub c_rs: f32,
    pub del_n: f32,
    pub m_0: f64,
    pub c_uc: f32,
    pub e: f64,
    pub c_us: f32,
    pub sqrt_a: f64,
    pub t_oe: u32,
    pub c_ic: f32,
    pub omega_0: f64,
    pub c_is: f32,
    pub i_0: f64,
    pub c_rc: f32,
    pub omega: f64,
    pub omegadot: f32,
    pub idot: f32,
    pub wn_t_oc: u16,
    pub wn_t_oe: u16,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = write_vec)]
    pub padding: Vec<u8>,
}

impl GPSNav {
    // CA or P on L2 codes
    pub const L2_RESERVED: u8 = 0;
    pub const L2_P_CODE: u8 = 1;
    pub const L2_CA_CODE: u8 = 2;
    
    // L2 data flag
    pub const L2_NAV_DATA_OFF: u8 = 0;
    pub const L2_NAV_DATA_ON: u8 = 1;
}