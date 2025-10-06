use binrw::binrw;
use alloc::vec::Vec;

// GPSCNav Block 4042
#[binrw]
#[derive(Debug)]
pub struct GPSCNav {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub flags: u8,
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
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub t_gd: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub isc_l1ca: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub isc_l2c: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub isc_l5i5: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub isc_l5q5: Option<f32>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl GPSCNav {
    // Flags bit definitions
    pub const FLAG_ALERT: u8 = 0x01;
    pub const FLAG_INTEGRITY_STATUS: u8 = 0x02;
    pub const FLAG_L2C_PHASING: u8 = 0x04;
    pub const FLAG_L2C_USED: u8 = 0x40;
    pub const FLAG_L5_USED: u8 = 0x80;
    
    pub fn is_alert(&self) -> bool {
        self.flags & Self::FLAG_ALERT != 0
    }
    
    pub fn is_l2c_used(&self) -> bool {
        self.flags & Self::FLAG_L2C_USED != 0
    }
    
    pub fn is_l5_used(&self) -> bool {
        self.flags & Self::FLAG_L5_USED != 0
    }
}