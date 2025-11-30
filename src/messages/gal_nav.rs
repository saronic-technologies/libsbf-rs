use binrw::binrw;
use alloc::vec::Vec;

// GALNav Block 4002
#[binrw]
#[derive(Debug, Clone)]
pub struct GALNav {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub source: u8,
    pub sqrt_a: f64,
    pub m_0: f64,
    pub e: f64,
    pub i_0: f64,
    pub omega: f64,
    pub omega_0: f64,
    pub omegadot: f32,
    pub idot: f32,
    pub del_n: f32,
    pub c_uc: f32,
    pub c_us: f32,
    pub c_rc: f32,
    pub c_rs: f32,
    pub c_ic: f32,
    pub c_is: f32,
    pub t_oe: u32,
    pub t_oc: u32,
    pub a_f2: f32,
    pub a_f1: f32,
    pub a_f0: f64,
    pub wn_t_oe: u16,
    pub wn_t_oc: u16,
    pub iod_nav: u16,
    pub health_ossol: u16,
    pub health_prs: u8,
    #[br(map = crate::do_not_use::map_u1)]
    #[bw(map = |x| crate::do_not_use::unmap_u1(x))]
    pub sisa_l1e5a: Option<u8>,
    #[br(map = crate::do_not_use::map_u1)]
    #[bw(map = |x| crate::do_not_use::unmap_u1(x))]
    pub sisa_l1e5b: Option<u8>,
    #[br(map = crate::do_not_use::map_u1)]
    #[bw(map = |x| crate::do_not_use::unmap_u1(x))]
    pub sisa_l1ae6a: Option<u8>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub bgd_l1e5a: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub bgd_l1e5b: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub bgd_l1ae6a: Option<f32>,
    #[br(map = crate::do_not_use::map_u1)]
    #[bw(map = |x| crate::do_not_use::unmap_u1(x))]
    pub cnav_enc: Option<u8>,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = crate::do_not_use::write_vec)]
    pub padding: Vec<u8>,
}

impl GALNav {
    // Source constants
    pub const SOURCE_INAV: u8 = 2;  // I/NAV (L1,E5b)
    pub const SOURCE_FNAV: u8 = 16; // F/NAV (L1,E5a)
    
    // Health_OSSOL bit masks
    pub const HEALTH_L1B_VALID: u16 = 0x0001;
    pub const HEALTH_L1B_DVS: u16 = 0x0002;
    pub const HEALTH_L1B_HS_MASK: u16 = 0x000C;
    pub const HEALTH_E5B_VALID: u16 = 0x0010;
    pub const HEALTH_E5B_DVS: u16 = 0x0020;
    pub const HEALTH_E5B_HS_MASK: u16 = 0x00C0;
    pub const HEALTH_E5A_VALID: u16 = 0x0100;
    pub const HEALTH_E5A_DVS: u16 = 0x0200;
    pub const HEALTH_E5A_HS_MASK: u16 = 0x0C00;
    
    // CNAVenc bit masks
    pub const CNAV_E6B_UNENCRYPTED: u8 = 0x01;
    pub const CNAV_E6C_UNENCRYPTED: u8 = 0x02;
}