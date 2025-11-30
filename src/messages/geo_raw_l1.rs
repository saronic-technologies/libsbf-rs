use binrw::binrw;

// GEORawL1 Block 4020
#[binrw]
#[derive(Debug, Clone)]
pub struct GEORawL1 {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub crc_passed: u8,
    pub viterbi_cnt: u8,
    pub source: u8,
    pub freq_nr: u8,
    pub rx_channel: u8,
    // 250 bits stored in 8 u32s (256 bits total, last 6 bits unused)
    pub nav_bits: [u32; 8],
}

impl GEORawL1 {
    // CRC status constants
    pub const CRC_FAILED: u8 = 0;
    pub const CRC_PASSED: u8 = 1;
}