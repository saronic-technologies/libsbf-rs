use binrw::binrw;
use alloc::vec::Vec;

// GALUtc Block 4031
#[binrw]
#[derive(Debug, Clone)]
pub struct GALUtc {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub source: u8,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub a_1: Option<f32>,
    #[br(map = crate::do_not_use::map_f8)]
    #[bw(map = |x| crate::do_not_use::unmap_f8(x))]
    pub a_0: Option<f64>,
    pub t_ot: u32,
    pub wn_ot: u8,
    pub del_t_ls: i8,
    pub wn_lsf: u8,
    pub dn: u8,
    pub del_t_lsf: i8,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = crate::do_not_use::write_vec)]
    pub padding: Vec<u8>,
}

impl GALUtc {
    // Source constants
    pub const SOURCE_INAV: u8 = 2;
    pub const SOURCE_FNAV: u8 = 16;
}