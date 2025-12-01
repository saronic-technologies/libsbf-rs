use binrw::binrw;
use crate::do_not_use::{map_u2, map_u4, unmap_u2, unmap_u4};

// Quality Indicator Block 4082
#[binrw]
#[derive(Debug, PartialEq)]
pub struct QualityInd {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub reserved: u8,
    #[br( if(n > 0))]
    pub indicator_1: Option<u16>,
    #[br(if(n > 1))]
    pub indicator_2: Option<u16>,
    #[br(if(n > 2))]
    pub indicator_3: Option<u16>,
    #[br(if(n > 3))]
    pub indicator_4: Option<u16>,
    #[br(if(n > 4))]
    pub indicator_5: Option<u16>,
    #[br(if(n > 5))]
    pub indicator_6: Option<u16>,
    #[br(if(n > 6))]
    pub indicator_7: Option<u16>,
    #[br( if(n > 7))]
    pub indicator_8: Option<u16>,
}