use binrw::binrw;
use alloc::vec::Vec;

// GALIon Block 4030
#[binrw]
#[derive(Debug)]
pub struct GALIon {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub source: u8,
    pub a_i0: f32,
    pub a_i1: f32,
    pub a_i2: f32,
    pub storm_flags: u8,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl GALIon {
    // Source constants
    pub const SOURCE_INAV: u8 = 2;
    pub const SOURCE_FNAV: u8 = 16;
    
    // Storm flag bits
    pub const STORM_FLAG_SF5: u8 = 0x01;
    pub const STORM_FLAG_SF4: u8 = 0x02;
    pub const STORM_FLAG_SF3: u8 = 0x04;
    pub const STORM_FLAG_SF2: u8 = 0x08;
    pub const STORM_FLAG_SF1: u8 = 0x10;
    
    pub fn is_storm_flag_sf5(&self) -> bool {
        self.storm_flags & Self::STORM_FLAG_SF5 != 0
    }
    
    pub fn is_storm_flag_sf4(&self) -> bool {
        self.storm_flags & Self::STORM_FLAG_SF4 != 0
    }
    
    pub fn is_storm_flag_sf3(&self) -> bool {
        self.storm_flags & Self::STORM_FLAG_SF3 != 0
    }
    
    pub fn is_storm_flag_sf2(&self) -> bool {
        self.storm_flags & Self::STORM_FLAG_SF2 != 0
    }
    
    pub fn is_storm_flag_sf1(&self) -> bool {
        self.storm_flags & Self::STORM_FLAG_SF1 != 0
    }
}