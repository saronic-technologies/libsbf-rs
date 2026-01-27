use alloc::vec::Vec;
use binrw::binrw;

// DiffCorrIn Block 5919
#[binrw]
#[derive(Debug)]
pub struct DiffCorrIn {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub mode: u8,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub source: Option<u8>,

    // The message content varies based on mode
    // binrw will read all remaining bytes
    #[br(parse_with = binrw::helpers::until_eof)]
    pub message_data: Vec<u8>,
}

// Mode constants for clarity
impl DiffCorrIn {
    pub const MODE_RTCM_V2: u8 = 0;
    pub const MODE_CMR_V2: u8 = 1;
    pub const MODE_RTCM_V3: u8 = 2;
    pub const MODE_RTCMV: u8 = 3;
    pub const MODE_SPARTN: u8 = 4;

    // Source constants
    pub const SOURCE_COM1: u8 = 0;
    pub const SOURCE_COM2: u8 = 1;
    pub const SOURCE_COM3: u8 = 2;
    pub const SOURCE_COM4: u8 = 3;
    pub const SOURCE_USB1: u8 = 4;
    pub const SOURCE_USB2: u8 = 5;
    pub const SOURCE_IP: u8 = 6;
    pub const SOURCE_SBF_FILE: u8 = 7;
    pub const SOURCE_LBAND: u8 = 8;
    pub const SOURCE_NTRIP: u8 = 9;
    pub const SOURCE_OTG1: u8 = 10;
    pub const SOURCE_OTG2: u8 = 11;
    pub const SOURCE_BLUETOOTH: u8 = 12;
    pub const SOURCE_UHF_MODEM: u8 = 15;
    pub const SOURCE_IPR: u8 = 16;
    pub const SOURCE_DIRECT_CALL: u8 = 17;
    pub const SOURCE_IPS: u8 = 18;
}
