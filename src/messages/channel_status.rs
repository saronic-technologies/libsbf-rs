use crate::binrw_util;
use crate::{NestedBlock, NestedHeader, SubBlock};
use alloc::vec::Vec;
use binrw::binrw;
use super::sat_visibility::RiseSet;

// ChannelStatus Block 4013
#[binrw]
#[derive(Clone, Debug)]
pub struct ChannelStatus {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb1_length: u8,
    pub sb2_length: u8,
    pub reserved: [u8; 3],
    #[br(args { count: usize::from(n), inner: (usize::from(sb1_length), usize::from(sb2_length)) },
         map = |v: Vec<NestedBlock<ChannelSatInfoHeader, ChannelStateInfo>>| v.into_iter().map(ChannelSatInfo::from).collect())]
    #[bw(args_raw = (usize::from(*sb1_length), usize::from(*sb2_length)),
         map = |v: &Vec<ChannelSatInfo>| v.iter().cloned().map(NestedBlock::from).collect::<Vec<NestedBlock<ChannelSatInfoHeader, ChannelStateInfo>>>())]
    pub sat_info: Vec<ChannelSatInfo>,
}

// First-level header of a ChannelSatInfo sub-block. Internal wire type; the flat
// public ChannelSatInfo is bridged from NestedBlock via From/Into.
#[binrw]
#[derive(Clone, Debug)]
struct ChannelSatInfoHeader {
    pub svid: u8,
    /// GLONASS frequency number with an offset of 8, from 1 to 21; reserved otherwise.
    #[br(map = binrw_util::map_u1_zero)]
    #[bw(map = binrw_util::unmap_u1_zero)]
    pub freq_nr: Option<u8>,
    pub reserved1: [u8; 2],
    /// Bit field: bits 0-8 azimuth in degrees, bits 14-15 rise/set indicator.
    pub azimuth_rise_set: u16,
    /// Sequence of 2-bit health status fields, one per signal.
    pub health_status: u16,
    /// Elevation in degrees relative to the local horizontal plane.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub elevation: Option<i8>,
    pub n2: u8,
    pub rx_channel: u8,
    pub reserved2: u8,
}

impl NestedHeader for ChannelSatInfoHeader {
    fn nested_count(&self) -> usize {
        usize::from(self.n2)
    }
}

/// One satellite's channel status: the first-level header fields followed by its
/// per-antenna state sub-blocks.
#[derive(Clone, Debug)]
pub struct ChannelSatInfo {
    pub svid: u8,
    pub freq_nr: Option<u8>,
    pub reserved1: [u8; 2],
    pub azimuth_rise_set: u16,
    pub health_status: u16,
    pub elevation: Option<i8>,
    pub n2: u8,
    pub rx_channel: u8,
    pub reserved2: u8,
    pub state_info: Vec<ChannelStateInfo>,
}

impl From<NestedBlock<ChannelSatInfoHeader, ChannelStateInfo>> for ChannelSatInfo {
    fn from(nb: NestedBlock<ChannelSatInfoHeader, ChannelStateInfo>) -> Self {
        ChannelSatInfo {
            svid: nb.header.svid,
            freq_nr: nb.header.freq_nr,
            reserved1: nb.header.reserved1,
            azimuth_rise_set: nb.header.azimuth_rise_set,
            health_status: nb.header.health_status,
            elevation: nb.header.elevation,
            n2: nb.header.n2,
            rx_channel: nb.header.rx_channel,
            reserved2: nb.header.reserved2,
            state_info: nb.items.into_iter().map(SubBlock::into_inner).collect(),
        }
    }
}

impl From<ChannelSatInfo> for NestedBlock<ChannelSatInfoHeader, ChannelStateInfo> {
    fn from(si: ChannelSatInfo) -> Self {
        NestedBlock {
            header: ChannelSatInfoHeader {
                svid: si.svid,
                freq_nr: si.freq_nr,
                reserved1: si.reserved1,
                azimuth_rise_set: si.azimuth_rise_set,
                health_status: si.health_status,
                elevation: si.elevation,
                n2: si.n2,
                rx_channel: si.rx_channel,
                reserved2: si.reserved2,
            },
            items: si.state_info.into_iter().map(SubBlock::from).collect(),
        }
    }
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
#[binrw]
#[derive(Clone, Debug)]
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
