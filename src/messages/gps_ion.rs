use binrw::binrw;
use alloc::vec::Vec;

// GPSIon Block 5893
#[binrw]
#[derive(Debug, Clone)]
pub struct GPSIon {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
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
    #[bw(write_with = crate::do_not_use::write_vec)]
    pub padding: Vec<u8>,
}