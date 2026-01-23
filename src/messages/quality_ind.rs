use binrw::binrw;
use alloc::vec::Vec;

// Quality Indicator Block 4082
#[binrw]
#[derive(Debug)]
pub struct QualityInd {
    #[br(map = |x| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub reserved: u8,
    #[br(count = n)]
    pub indicators: Vec<u16>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl PartialEq for QualityInd {
    fn eq(&self, other: &Self) -> bool {
        self.tow == other.tow
            && self.wnc == other.wnc
            && self.n == other.n
            && self.reserved == other.reserved
            && self.indicators == other.indicators
        // Note: padding is intentionally not compared as it's not part of the logical message content
    }
}