use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_u2, map_u4, unmap_u2, unmap_u4, write_vec};

// Meas3Ranges Block 4109
// NOTE: The exact structure of this message is not documented in the SBF reference.
// The documentation states that the reference C implementation should be used to 
// parse these messages. For now, we store the raw bytes.
#[binrw]
#[derive(Debug, Clone)]
pub struct Meas3Ranges {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    
    // The rest of the message is undocumented and requires the C implementation
    // to properly parse. We store the raw bytes for future processing.
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = write_vec)]
    pub raw_data: Vec<u8>,
}