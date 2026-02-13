use alloc::vec::Vec;
use binrw::BinRead;

/// RFBand sub-block: interference info for a single RF band.
#[derive(Debug, BinRead)]
pub struct RFBand {
    /// Center frequency of the RF band (Hz).
    pub frequency: u32,
    /// Bandwidth of the RF band (kHz).
    pub bandwidth: u16,
    /// Info byte. Bits 0-3: mode (1=manual notch, 2=detected+mitigated, 8=detected no mitigation).
    /// Bits 6-7: antenna ID (0=main, 1=aux1, 2=aux2).
    pub info: u8,
    /// Estimated interference power (dBm). 0 if unknown.
    #[br(map = |x: i8| if x == 0 { None } else { Some(x) })]
    pub power: Option<i8>,
}

impl RFBand {
    /// Interference mitigation mode (bits 0-3).
    pub fn mode(&self) -> u8 {
        self.info & 0x0F
    }

    /// Antenna ID (bits 6-7): 0=main, 1=aux1, 2=aux2.
    pub fn antenna_id(&self) -> u8 {
        (self.info >> 6) & 0x03
    }

    /// Whether the receiver detected and successfully mitigated interference.
    pub fn mitigated(&self) -> bool {
        self.mode() == 2
    }

    /// Whether interference was detected but not mitigated.
    pub fn detected_unmitigated(&self) -> bool {
        self.mode() == 8
    }
}

// RFStatus Block 4092
#[derive(Debug, BinRead)]
pub struct RFStatus {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    n: u8,
    pub sb_length: u8,
    /// Bit 0: GNSS signals may not be authentic (spoofing/simulator).
    /// Bit 1: NMA check failed (e.g. Galileo OSNMA).
    pub flags: u8,
    _reserved: [u8; 3],
    #[br(count = usize::from(n))]
    pub bands: Vec<RFBand>,
    #[br(parse_with = binrw::helpers::until_eof)]
    _padding: Vec<u8>,
}

impl RFStatus {
    /// Number of RF bands with interference info.
    pub fn num_bands(&self) -> u8 {
        self.n
    }

    /// Whether the receiver suspects non-authentic GNSS signals (spoofing).
    pub fn spoofing_detected(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Whether NMA (Navigation Message Authentication) verification failed.
    pub fn nma_failed(&self) -> bool {
        self.flags & 0x02 != 0
    }
}
