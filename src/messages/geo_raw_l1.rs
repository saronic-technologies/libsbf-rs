use binrw::binrw;

// GEORawL1 Block 4020
#[binrw]
#[derive(Debug)]
pub struct GEORawL1 {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
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
