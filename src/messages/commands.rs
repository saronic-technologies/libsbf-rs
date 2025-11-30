use binrw::binrw;
use alloc::vec::Vec;

// Commands Block 4015
#[binrw]
#[derive(Debug, Clone)]
pub struct Commands {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub reserved: [u8; 2],
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = crate::do_not_use::write_vec)]
    pub cmd_data: Vec<u8>,
}