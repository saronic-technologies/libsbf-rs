use alloc::vec::Vec;
use binrw::BinRead;

use super::sat_visibility::RiseSet;

// ChannelStatus Block 4013
#[derive(BinRead, Clone, Debug)]
pub struct ChannelStatus {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb1_length: u8,
    pub sb2_length: u8,
    pub reserved: [u8; 3],
    #[br(count = usize::from(n))]
    pub sat_info: Vec<ChannelSatInfo>,
}

// ChannelSatInfo sub-block
#[derive(BinRead, Clone, Debug)]
pub struct ChannelSatInfo {
    pub svid: u8,
    /// GLONASS frequency number with an offset of 8, from 1 to 21; reserved otherwise.
    #[br(map = |x: u8| if x == 0 { None } else { Some(x) })]
    pub freq_nr: Option<u8>,
    pub reserved1: [u8; 2],
    /// Bit field: bits 0-8 azimuth in degrees, bits 14-15 rise/set indicator.
    pub azimuth_rise_set: u16,
    /// Sequence of 2-bit health status fields, one per signal.
    pub health_status: u16,
    /// Elevation in degrees relative to the local horizontal plane.
    #[br(map = |x: i8| if x == crate::DO_NOT_USE_I1 { None } else { Some(x) })]
    pub elevation: Option<i8>,
    pub n2: u8,
    pub rx_channel: u8,
    pub reserved2: u8,
    #[br(count = usize::from(n2))]
    pub state_info: Vec<ChannelStateInfo>,
}

impl ChannelSatInfo {
    /// Azimuth in degrees from bits 0-8, 0 is North; None when the raw value is 511.
    pub fn azimuth(&self) -> Option<u16> {
        let value = self.azimuth_rise_set & 0x1FF;
        if value == 511 {
            None
        } else {
            Some(value)
        }
    }

    /// Rise/set indicator from bits 14-15 of the azimuth field.
    pub fn rise_set(&self) -> RiseSet {
        RiseSet::from((self.azimuth_rise_set >> 14) as u8)
    }
}

// ChannelStateInfo sub-sub-block
#[derive(BinRead, Clone, Debug)]
pub struct ChannelStateInfo {
    /// Antenna number, 0 for the main antenna.
    pub antenna: u8,
    pub reserved: u8,
    /// Sequence of 2-bit tracking status fields: 0 idle, 1 search, 2 sync, 3 tracking.
    pub tracking_status: u16,
    /// Sequence of 2-bit PVT status fields: 0 not used, 1 waiting, 2 used, 3 rejected.
    pub pvt_status: u16,
    pub pvt_info: u16,
}
