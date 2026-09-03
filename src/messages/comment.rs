use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// Comment Block 5936
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct Comment {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    /// Length of `comment` in bytes. The string is not NUL-terminated.
    pub comment_ln: u16,
    #[br(count = usize::from(comment_ln))]
    pub comment: Vec<u8>,
}
