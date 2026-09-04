use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// Commands Block 4015
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct Commands {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub reserved: [u8; 2],
    #[br(parse_with = binrw::helpers::until_eof)]
    pub cmd_data: Vec<u8>,
}
