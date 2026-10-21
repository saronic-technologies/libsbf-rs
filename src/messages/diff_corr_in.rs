use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use num_enum::{FromPrimitive, IntoPrimitive};

/// Format of a differential correction message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DiffCorrMode {
    RtcmV2 = 0,
    CmrV2 = 1,
    RtcmV3 = 2,
    /// Proprietary variant of RTCM2.
    Rtcmv = 3,
    Spartn = 4,
    /// Unrecognized correction format.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Receiver connection a differential correction message arrived on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DiffCorrSource {
    Com1 = 0,
    Com2 = 1,
    Com3 = 2,
    Com4 = 3,
    Usb1 = 4,
    Usb2 = 5,
    /// IP connection.
    Ip = 6,
    SbfFile = 7,
    /// Message decoded by the built-in L-band demodulator.
    LBand = 8,
    Ntrip = 9,
    Otg1 = 10,
    Otg2 = 11,
    Bluetooth = 12,
    UhfModem = 15,
    /// IPR connection.
    Ipr = 16,
    DirectCall = 17,
    /// IPS connection.
    Ips = 18,
    /// Message generated internally by SSR2OBS conversion.
    Ssr2Obs = 19,
    /// Unrecognized source code.
    #[num_enum(catch_all)]
    Unknown(u8),
}

// DiffCorrIn Block 5919
#[binrw]
#[derive(Clone, Debug)]
pub struct DiffCorrIn {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = |x: u8| DiffCorrMode::from(x))]
    #[bw(map = |x: &DiffCorrMode| u8::from(*x))]
    pub mode: DiffCorrMode,
    #[br(map = binrw_util::map_u1_enum_opt)]
    #[bw(map = binrw_util::unmap_u1_enum_opt)]
    pub source: Option<DiffCorrSource>,

    // The message content varies based on mode
    // binrw will read all remaining bytes
    #[br(parse_with = binrw::helpers::until_eof)]
    pub message_data: Vec<u8>,
}
