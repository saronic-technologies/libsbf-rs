use crate::util::binrw::{map_u2, map_u4, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// EndOfAtt Block 5943
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct EndOfAtt {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
