use alloc::vec::Vec;
use binrw::binrw;

// EndOfAtt Block 5943
#[binrw]
#[derive(Clone, Debug)]
pub struct EndOfAtt {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
