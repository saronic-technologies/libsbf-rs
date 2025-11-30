use binrw::binrw;
use alloc::vec::Vec;

// Meas3Ranges Block 4109
// NOTE: The exact structure of this message is not documented in the SBF reference.
// The documentation states that the reference C implementation should be used to 
// parse these messages. For now, we store the raw bytes.
#[binrw]
#[derive(Debug, Clone)]
pub struct Meas3Ranges {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    
    // The rest of the message is undocumented and requires the C implementation
    // to properly parse. We store the raw bytes for future processing.
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = crate::do_not_use::write_vec)]
    pub raw_data: Vec<u8>,
}