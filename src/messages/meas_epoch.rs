use crate::binrw_util;
use crate::{NestedBlock, NestedHeader, SubBlock};
use alloc::vec::Vec;
use binrw::binrw;
use bitflags::bitflags;

bitflags! {
    /// CommonFlags bit field of the [`MeasEpoch`] block.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MeasEpochCommonFlags: u8 {
        /// Bit 0: multipath mitigation enabled.
        const MULTIPATH_MITIGATION = 1 << 0;
        /// Bit 1: at least one of the code measurements is smoothed.
        const SMOOTHING = 1 << 1;
        /// Bit 3: clock steering active.
        const CLOCK_STEERING = 1 << 3;
        /// Bit 5: receiver in high-dynamics mode.
        const HIGH_DYNAMICS = 1 << 5;
        /// Bit 6: Galileo E6 measurements obtained from the E6B signal
        /// instead of the default E6C.
        const E6B_USED = 1 << 6;
        /// Bit 7: measurements scrambled since the "Measurement Availability"
        /// permission is not granted.
        const SCRAMBLED = 1 << 7;
    }
}

// MeasEpoch Block 4027
#[binrw]
#[derive(Clone, Debug)]
pub struct MeasEpoch {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub n1: u8,
    pub sb1_length: u8,
    pub sb2_length: u8,
    #[br(map = |x: u8| MeasEpochCommonFlags::from_bits_retain(x))]
    #[bw(map = |x: &MeasEpochCommonFlags| x.bits())]
    pub common_flags: MeasEpochCommonFlags,
    pub cum_clk_jumps: u8,
    pub rev1: u8,
    #[br(args { count: usize::from(n1), inner: (usize::from(sb1_length), usize::from(sb2_length)) },
         map = |v: Vec<NestedBlock<MeasEpochChannelType1Header, MeasEpochChannelType2>>| v.into_iter().map(MeasEpochChannelType1::from).collect())]
    #[bw(args_raw = (usize::from(*sb1_length), usize::from(*sb2_length)),
         map = |v: &Vec<MeasEpochChannelType1>| v.iter().cloned().map(NestedBlock::from).collect::<Vec<NestedBlock<MeasEpochChannelType1Header, MeasEpochChannelType2>>>())]
    pub channel_type1: Vec<MeasEpochChannelType1>,
}

// First-level header of a MeasEpochChannelType1 sub-block. Internal wire type;
// the flat public MeasEpochChannelType1 is bridged from NestedBlock.
#[binrw]
#[derive(Clone, Debug)]
struct MeasEpochChannelType1Header {
    pub rx_channel: u8,
    pub type_field: u8,
    pub svid: u8,
    pub misc: u8,
    pub code_lsb: u32,
    #[br(map = |x: i32| if x == -2147483648 { None } else { Some(x) })]
    #[bw(map = |x: &Option<i32>| x.unwrap_or(-2147483648))]
    pub doppler: Option<i32>,
    pub carrier_lsb: u16,
    pub carrier_msb: i8,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub cn0: Option<u8>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub lock_time: Option<u16>,
    pub obs_info: u8,
    pub n2: u8,
}

impl NestedHeader for MeasEpochChannelType1Header {
    fn nested_count(&self) -> usize {
        usize::from(self.n2)
    }
}

#[derive(Clone, Debug)]
pub struct MeasEpochChannelType1 {
    pub rx_channel: u8,
    pub type_field: u8,
    pub svid: u8,
    pub misc: u8,
    pub code_lsb: u32,
    pub doppler: Option<i32>,
    pub carrier_lsb: u16,
    pub carrier_msb: i8,
    pub cn0: Option<u8>,
    pub lock_time: Option<u16>,
    pub obs_info: u8,
    pub n2: u8,
    pub channel_type2: Vec<MeasEpochChannelType2>,
}

impl From<NestedBlock<MeasEpochChannelType1Header, MeasEpochChannelType2>> for MeasEpochChannelType1 {
    fn from(nb: NestedBlock<MeasEpochChannelType1Header, MeasEpochChannelType2>) -> Self {
        MeasEpochChannelType1 {
            rx_channel: nb.header.rx_channel,
            type_field: nb.header.type_field,
            svid: nb.header.svid,
            misc: nb.header.misc,
            code_lsb: nb.header.code_lsb,
            doppler: nb.header.doppler,
            carrier_lsb: nb.header.carrier_lsb,
            carrier_msb: nb.header.carrier_msb,
            cn0: nb.header.cn0,
            lock_time: nb.header.lock_time,
            obs_info: nb.header.obs_info,
            n2: nb.header.n2,
            channel_type2: nb.items.into_iter().map(SubBlock::into_inner).collect(),
        }
    }
}

impl From<MeasEpochChannelType1> for NestedBlock<MeasEpochChannelType1Header, MeasEpochChannelType2> {
    fn from(ct1: MeasEpochChannelType1) -> Self {
        NestedBlock {
            header: MeasEpochChannelType1Header {
                rx_channel: ct1.rx_channel,
                type_field: ct1.type_field,
                svid: ct1.svid,
                misc: ct1.misc,
                code_lsb: ct1.code_lsb,
                doppler: ct1.doppler,
                carrier_lsb: ct1.carrier_lsb,
                carrier_msb: ct1.carrier_msb,
                cn0: ct1.cn0,
                lock_time: ct1.lock_time,
                obs_info: ct1.obs_info,
                n2: ct1.n2,
            },
            items: ct1.channel_type2.into_iter().map(SubBlock::from).collect(),
        }
    }
}

impl MeasEpochChannelType1 {
    /// Bit 0 of obs_info: the pseudorange is smoothed.
    pub fn smoothed(&self) -> bool {
        self.obs_info & 1 != 0
    }

    /// Bit 2 of obs_info: the carrier phase has a half-cycle ambiguity.
    pub fn half_cycle_ambiguity(&self) -> bool {
        self.obs_info & (1 << 2) != 0
    }

    /// Antenna ID from bits 5-7 of the type field: 0 main, 1 Aux1, 2 Aux2.
    pub fn antenna_id(&self) -> u8 {
        self.type_field >> 5
    }

    /// Signal number per section 4.1.10 of the reference guide: bits 0-4 of
    /// the type field, or 32 plus bits 3-7 of obs_info when those read 31.
    pub fn signal_number(&self) -> u8 {
        let sig_idx_lo = self.type_field & 0x1F;
        if sig_idx_lo == 31 {
            32 + (self.obs_info >> 3)
        } else {
            sig_idx_lo
        }
    }

    /// GLONASS frequency number from -7 to 6, from bits 3-7 of obs_info.
    /// Available when the signal index selects a GLONASS signal.
    pub fn glonass_freq_nr(&self) -> Option<i8> {
        let sig_idx_lo = self.type_field & 0x1F;
        if (8..=11).contains(&sig_idx_lo) {
            Some((self.obs_info >> 3) as i8 - 8)
        } else {
            None
        }
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct MeasEpochChannelType2 {
    pub type_field: u8,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub lock_time: Option<u8>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub cn0: Option<u8>,
    pub offsets_msb: u8,
    pub carrier_msb: i8,
    pub obs_info: u8,
    pub code_offset_lsb: u16,
    pub carrier_lsb: u16,
    pub doppler_offset_lsb: u16,
}

impl MeasEpochChannelType2 {
    /// Bit 0 of obs_info: the pseudorange is smoothed.
    pub fn smoothed(&self) -> bool {
        self.obs_info & 1 != 0
    }

    /// Bit 2 of obs_info: the carrier phase has a half-cycle ambiguity.
    pub fn half_cycle_ambiguity(&self) -> bool {
        self.obs_info & (1 << 2) != 0
    }

    /// Antenna ID from bits 5-7 of the type field: 0 main, 1 Aux1, 2 Aux2.
    pub fn antenna_id(&self) -> u8 {
        self.type_field >> 5
    }

    /// Signal number per section 4.1.10 of the reference guide: bits 0-4 of
    /// the type field, or 32 plus bits 3-7 of obs_info when those read 31.
    pub fn signal_number(&self) -> u8 {
        let sig_idx_lo = self.type_field & 0x1F;
        if sig_idx_lo == 31 {
            32 + (self.obs_info >> 3)
        } else {
            sig_idx_lo
        }
    }
}
