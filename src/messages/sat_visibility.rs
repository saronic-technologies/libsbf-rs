use alloc::vec::Vec;
use binrw::BinRead;

// SatVisibility Block 4012
#[derive(Clone, Debug, BinRead)]
pub struct SatVisibility {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(count = usize::from(n))]
    pub satellites: Vec<SatInfo>,
}

// SatInfo sub-block
#[derive(Clone, Debug, BinRead)]
pub struct SatInfo {
    pub svid: u8,
    /// GLONASS frequency number with an offset of 8, from 1 to 21; reserved otherwise.
    #[br(map = |x: u8| if x == 0 { None } else { Some(x) })]
    pub freq_nr: Option<u8>,
    /// Azimuth in 0.01 degrees, 0 is North and increasing towards East.
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub azimuth: Option<u16>,
    /// Elevation in 0.01 degrees relative to the local horizontal plane.
    #[br(map = |x: i16| if x == crate::DO_NOT_USE_I2 { None } else { Some(x) })]
    pub elevation: Option<i16>,
    #[br(map = |x: u8| RiseSet::from(x))]
    pub rise_set: RiseSet,
    /// Source of the visibility info: 1 almanac, 2 ephemeris, 255 unknown.
    pub satellite_info: u8,
}

/// Rise/set state of a satellite.
#[derive(Clone, Copy, Debug)]
pub enum RiseSet {
    Setting,
    Rising,
    Unknown,
    Other(u8),
}

impl From<u8> for RiseSet {
    fn from(value: u8) -> Self {
        match value {
            0 => RiseSet::Setting,
            1 => RiseSet::Rising,
            // 3 in the 2-bit ChannelStatus field, 255 in SatVisibility.
            3 | 255 => RiseSet::Unknown,
            other => RiseSet::Other(other),
        }
    }
}
