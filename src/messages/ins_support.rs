use crate::util::binrw::{map_u2, map_u4, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// INSSupport Block 4077
// NOTE: The exact structure of this message is not documented in the SBF reference.
// The documentation states that the reference C implementation should be used to
// parse these messages. For now, we store the raw bytes.
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct INSSupport {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,

    // The rest of the message is undocumented and requires the C implementation
    // to properly parse. We store the raw bytes for future processing.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub raw_data: Vec<u8>,
}
