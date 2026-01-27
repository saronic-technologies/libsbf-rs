use alloc::vec::Vec;
use binrw::binrw;

// GPSUtc Block 5894
#[binrw]
#[derive(Debug)]
pub struct GPSUtc {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub reserved: u8,
    pub a_1: f32,
    pub a_0: f64,
    pub t_ot: u32,
    pub wn_t: u8,
    pub del_t_ls: i8,
    pub wn_lsf: u8,
    pub dn: u8,
    pub del_t_lsf: i8,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
