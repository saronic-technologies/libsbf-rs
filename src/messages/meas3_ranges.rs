use binrw::binrw;
use alloc::vec::Vec;

// Meas3Ranges Block 4109
// NOTE: The exact structure of this message is not documented in the SBF reference.
// The documentation states that the reference C implementation should be used to 
// parse these messages. For now, we store the raw bytes.
#[binrw]
#[derive(Debug)]
pub struct Meas3Ranges {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    
    // The rest of the message is undocumented and requires the C implementation
    // to properly parse. We store the raw bytes for future processing.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub raw_data: Vec<u8>,
}