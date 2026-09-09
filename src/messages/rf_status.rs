use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use bitflags::bitflags;
use num_enum::{FromPrimitive, IntoPrimitive};

bitflags! {
    /// Flags bit field of the [`RFStatus`] block.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RFStatusFlags: u8 {
        /// Bit 0: the GNSS signals may not be authentic; the receiver may be
        /// connected to a simulator or subject to a spoofing attack.
        const SIG_AUTH_ALERT = 1 << 0;
        /// Bit 1: a non-authentic navigation message was detected by NMA
        /// checks such as Galileo OSNMA.
        const NAV_MSG_AUTH_ALERT = 1 << 1;
    }
}

/// Interference mitigation mode of an RF band, from bits 0-3 of the info
/// field.
#[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum RFBandMode {
    /// The band is suppressed by a notch filter set manually by user command.
    ManualNotch = 1,
    /// The receiver detected interference in the band and canceled it.
    Mitigated = 2,
    /// The receiver detected interference in the band; no mitigation applied.
    Unmitigated = 8,
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// RFBand sub-block: interference info for a single RF band.
#[binrw]
#[derive(Clone, Debug)]
pub struct RFBand {
    /// Center frequency of the RF band (Hz).
    pub frequency: u32,
    /// Bandwidth of the RF band (kHz).
    pub bandwidth: u16,
    /// Bit field: mode in bits 0-3, antenna ID in bits 6-7.
    pub info: u8,
    /// Estimated interference power (dBm). 0 if unknown.
    #[br(map = binrw_util::map_i1_zero)]
    #[bw(map = binrw_util::unmap_i1_zero)]
    pub power: Option<i8>,
}

impl RFBand {
    /// Interference mitigation mode, from bits 0-3 of info.
    pub fn mode(&self) -> RFBandMode {
        RFBandMode::from(self.info & 0x0F)
    }

    /// Antenna ID from bits 6-7 of info: 0 main, 1 Aux1, 2 Aux2.
    pub fn antenna_id(&self) -> u8 {
        (self.info >> 6) & 0x03
    }
}

// RFStatus Block 4092
#[binrw]
#[derive(Clone, Debug)]
pub struct RFStatus {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    n: u8,
    pub sb_length: u8,
    #[br(map = |x: u8| RFStatusFlags::from_bits_retain(x))]
    #[bw(map = |x: &RFStatusFlags| x.bits())]
    pub flags: RFStatusFlags,
    _reserved: [u8; 3],
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = binrw_util::unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = binrw_util::wrap_subblocks)]
    pub bands: Vec<RFBand>,
    #[br(parse_with = binrw::helpers::until_eof)]
    _padding: Vec<u8>,
}

impl RFStatus {
    /// Number of RF bands with interference info.
    pub fn num_bands(&self) -> u8 {
        self.n
    }
}
