use binrw::binrw;
use alloc::vec::Vec;

// Commands Block 4015
#[binrw]
#[derive(Debug)]
pub struct Commands {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub reserved: [u8; 2],
    #[br(parse_with = binrw::helpers::until_eof)]
    pub cmd_data: Vec<u8>,
}