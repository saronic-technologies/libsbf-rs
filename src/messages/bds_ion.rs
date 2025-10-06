use binrw::binrw;
use alloc::vec::Vec;

// BDSIon Block 4120
#[binrw]
#[derive(Debug)]
pub struct BDSIon {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub reserved: u8,
    pub alpha_0: f32,
    pub alpha_1: f32,
    pub alpha_2: f32,
    pub alpha_3: f32,
    pub beta_0: f32,
    pub beta_1: f32,
    pub beta_2: f32,
    pub beta_3: f32,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}