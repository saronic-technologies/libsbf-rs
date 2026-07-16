use alloc::vec::Vec;
use binrw::binrw;

// Comment Block 5936
#[binrw]
#[derive(Clone, Debug)]
pub struct Comment {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(crate::DO_NOT_USE_U4))]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub wnc: Option<u16>,
    /// Length of `comment` in bytes. The string is not NUL-terminated.
    pub comment_ln: u16,
    #[br(count = usize::from(comment_ln))]
    pub comment: Vec<u8>,
}
