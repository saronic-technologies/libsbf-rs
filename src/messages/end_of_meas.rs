use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

// EndOfMeas Block 5922
#[binrw]
#[derive(Clone, Debug)]
pub struct EndOfMeas {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
