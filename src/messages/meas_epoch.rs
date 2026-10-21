use crate::util::{
    binrw::{
        map_u1, map_u2, map_u4, unmap_u1, unmap_u2, unmap_u4, NestedBlock, NestedHeader, SubBlock,
    },
    signal::{carrier_frequency_hz, cn0_db_hz, glonass_frequency_number, signal_number},
};
use alloc::vec::Vec;
use binrw::binrw;
use bitflags::bitflags;
use core::ops::RangeInclusive;

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

// SigIdxLo values that denote GLONASS signals.
const GLONASS_SIG_NRS: RangeInclusive<u8> = 8..=11;
// Speed of light in vacuum, in m/s.
const SPEED_OF_LIGHT_M_S: f64 = 299_792_458.0;
// Step of the pseudorange and of the code offset, in m.
const CODE_STEP_M: f64 = 0.001;
// Step of the Doppler and of the Doppler offset, in Hz.
const DOPPLER_STEP_HZ: f64 = 0.0001;
// Step of the carrier phase relative to the pseudorange, in cycles.
const CARRIER_STEP_CYCLES: f64 = 0.001;
// CarrierMSB that, with a CarrierLSB of 0, marks the carrier phase invalid.
const CARRIER_MSB_INVALID: i8 = -128;
// CodeOffsetMSB that, with a CodeOffsetLSB of 0, marks the code offset invalid.
const CODE_OFFSET_MSB_INVALID: i32 = -4;
// DopplerOffsetMSB that, with a DopplerOffsetLSB of 0, marks the Doppler offset invalid.
const DOPPLER_OFFSET_MSB_INVALID: i32 = -16;

bitflags! {
    /// ObsInfo bit field of the MeasEpoch channel sub-blocks.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MeasEpochObsInfo: u8 {
        /// Bit 0: the pseudorange is smoothed.
        const SMOOTHED = 1 << 0;
        /// Bit 2: the carrier phase has a half-cycle ambiguity.
        const HALF_CYCLE_AMBIGUITY = 1 << 2;
        /// Mask for bits 3-7: the signal number extension or GLONASS
        /// frequency number selected by the type field, decoded by
        /// [`MeasEpochChannelType1::signal_number`] and
        /// [`MeasEpochChannelType1::glonass_freq_nr`].
        const SIG_IDX_HI_MASK = 0b11111 << Self::SIG_IDX_HI_SHIFT;
    }
}

impl MeasEpochObsInfo {
    const SIG_IDX_HI_SHIFT: u32 = 3;

    /// Value of bits 3-7. The type field selects its meaning.
    pub fn sig_idx_hi(&self) -> u8 {
        self.intersection(Self::SIG_IDX_HI_MASK).bits() >> Self::SIG_IDX_HI_SHIFT
    }
}

// MeasEpoch Block 4027
#[binrw]
#[derive(Clone, Debug)]
pub struct MeasEpoch {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
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
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub cn0: Option<u8>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub lock_time: Option<u16>,
    #[br(map = MeasEpochObsInfo::from_bits_retain)]
    #[bw(map = |x: &MeasEpochObsInfo| x.bits())]
    pub obs_info: MeasEpochObsInfo,
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
    pub obs_info: MeasEpochObsInfo,
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
    /// Antenna ID from bits 5-7 of the type field: 0 main, 1 Aux1, 2 Aux2.
    pub fn antenna_id(&self) -> u8 {
        self.type_field >> 5
    }

    /// Signal number per section 4.1.10 of the reference guide: bits 0-4 of
    /// the type field, extended through obs_info when they read the escape
    /// value.
    pub fn signal_number(&self) -> u8 {
        signal_number(self.type_field, self.obs_info.sig_idx_hi())
    }

    /// C/N0 in dB-Hz, None at the Do-Not-Use value.
    pub fn cn0_db_hz(&self) -> Option<f32> {
        self.cn0.map(|cn0| cn0_db_hz(cn0, self.signal_number()))
    }

    /// GLONASS frequency number, from -7 to 6 for satellites launched after 2005 and up to 13
    /// before that (supported in firmware prior to 1.5.x), from bits 3-7 of obs_info.
    /// Available when the signal index selects a GLONASS signal.
    pub fn glonass_freq_nr(&self) -> Option<i8> {
        let sig_idx_lo = self.type_field & 0x1F;
        if GLONASS_SIG_NRS.contains(&sig_idx_lo) {
            Some(glonass_frequency_number(self.obs_info.sig_idx_hi()))
        } else {
            None
        }
    }

    /// Pseudorange in meters, from CodeMSB in bits 0-3 of misc and CodeLSB. None when both are
    /// 0, which marks the pseudorange invalid.
    pub fn pseudorange_m(&self) -> Option<f64> {
        let code = (u64::from(self.misc & 0x0F) << 32) | u64::from(self.code_lsb);
        (code != 0).then_some(code as f64 * CODE_STEP_M)
    }

    /// Carrier Doppler in Hz, positive for approaching satellites.
    pub fn doppler_hz(&self) -> Option<f64> {
        self.doppler
            .map(|doppler| f64::from(doppler) * DOPPLER_STEP_HZ)
    }

    /// Carrier frequency of the signal in Hz. None for a signal without a fixed frequency.
    pub fn carrier_frequency_hz(&self) -> Option<f64> {
        carrier_frequency_hz(self.signal_number(), self.glonass_freq_nr())
    }

    /// Full carrier phase in cycles. None when the carrier phase is invalid, or when the
    /// pseudorange or the carrier frequency is not available.
    pub fn carrier_phase_cycles(&self) -> Option<f64> {
        carrier_phase_cycles(
            self.pseudorange_m()?,
            self.carrier_frequency_hz()?,
            self.carrier_msb,
            self.carrier_lsb,
        )
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct MeasEpochChannelType2 {
    pub type_field: u8,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub lock_time: Option<u8>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub cn0: Option<u8>,
    pub offsets_msb: u8,
    pub carrier_msb: i8,
    #[br(map = MeasEpochObsInfo::from_bits_retain)]
    #[bw(map = |x: &MeasEpochObsInfo| x.bits())]
    pub obs_info: MeasEpochObsInfo,
    pub code_offset_lsb: u16,
    pub carrier_lsb: u16,
    pub doppler_offset_lsb: u16,
}

impl MeasEpochChannelType2 {
    /// Antenna ID from bits 5-7 of the type field: 0 main, 1 Aux1, 2 Aux2.
    pub fn antenna_id(&self) -> u8 {
        self.type_field >> 5
    }

    /// Signal number per section 4.1.10 of the reference guide: bits 0-4 of
    /// the type field, extended through obs_info when they read the escape
    /// value.
    pub fn signal_number(&self) -> u8 {
        signal_number(self.type_field, self.obs_info.sig_idx_hi())
    }

    /// C/N0 in dB-Hz, None at the Do-Not-Use value.
    pub fn cn0_db_hz(&self) -> Option<f32> {
        self.cn0.map(|cn0| cn0_db_hz(cn0, self.signal_number()))
    }

    /// Pseudorange in meters, from the code offset relative to the pseudorange of the `parent`
    /// sub-block. None when the code offset is invalid or the parent pseudorange is not
    /// available.
    pub fn pseudorange_m(&self, parent: &MeasEpochChannelType1) -> Option<f64> {
        let msb = i32::from(((self.offsets_msb << 5) as i8) >> 5);
        if msb == CODE_OFFSET_MSB_INVALID && self.code_offset_lsb == 0 {
            return None;
        }
        let offset = (msb << 16) + i32::from(self.code_offset_lsb);
        Some(parent.pseudorange_m()? + f64::from(offset) * CODE_STEP_M)
    }

    /// Carrier Doppler in Hz, from the Doppler offset relative to the Doppler of the `parent`
    /// sub-block, scaled by the ratio of the two carrier frequencies. None when the Doppler
    /// offset is invalid, or when the parent Doppler or either carrier frequency is not
    /// available.
    pub fn doppler_hz(&self, parent: &MeasEpochChannelType1) -> Option<f64> {
        let msb = i32::from((self.offsets_msb as i8) >> 3);
        if msb == DOPPLER_OFFSET_MSB_INVALID && self.doppler_offset_lsb == 0 {
            return None;
        }
        let offset = (msb << 16) + i32::from(self.doppler_offset_lsb);
        let ratio = self.carrier_frequency_hz(parent)? / parent.carrier_frequency_hz()?;
        Some(parent.doppler_hz()? * ratio + f64::from(offset) * DOPPLER_STEP_HZ)
    }

    /// Carrier frequency of the signal in Hz, using the GLONASS frequency number of the
    /// `parent` sub-block, since both observe the same satellite. None for a signal without
    /// a fixed frequency.
    pub fn carrier_frequency_hz(&self, parent: &MeasEpochChannelType1) -> Option<f64> {
        carrier_frequency_hz(self.signal_number(), parent.glonass_freq_nr())
    }

    /// Full carrier phase in cycles. None when the carrier phase is invalid, or when the
    /// pseudorange or the carrier frequency is not available.
    pub fn carrier_phase_cycles(&self, parent: &MeasEpochChannelType1) -> Option<f64> {
        carrier_phase_cycles(
            self.pseudorange_m(parent)?,
            self.carrier_frequency_hz(parent)?,
            self.carrier_msb,
            self.carrier_lsb,
        )
    }
}

// Full carrier phase in cycles from the pseudorange, the carrier frequency and the carrier
// phase relative to the pseudorange. None at the invalid marker of the relative phase.
fn carrier_phase_cycles(pseudorange_m: f64, frequency_hz: f64, msb: i8, lsb: u16) -> Option<f64> {
    if msb == CARRIER_MSB_INVALID && lsb == 0 {
        return None;
    }
    let relative = (i32::from(msb) << 16) + i32::from(lsb);
    Some(
        pseudorange_m * frequency_hz / SPEED_OF_LIGHT_M_S
            + f64::from(relative) * CARRIER_STEP_CYCLES,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        MeasEpochChannelType1, MeasEpochChannelType2, MeasEpochObsInfo, SPEED_OF_LIGHT_M_S,
    };
    use alloc::vec::Vec;

    // Signal numbers of GPS L1CA and L2C, and their carrier frequencies in Hz.
    const GPS_L1CA: u8 = 0;
    const GPS_L2C: u8 = 3;
    const GPS_L1_HZ: f64 = 1_575.42e6;
    const GPS_L2_HZ: f64 = 1_227.60e6;
    // Allowed rounding error of a derived measurement.
    const EPSILON: f64 = 1e-9;

    fn type1(misc: u8, code_lsb: u32, carrier_msb: i8, carrier_lsb: u16) -> MeasEpochChannelType1 {
        MeasEpochChannelType1 {
            rx_channel: 1,
            type_field: GPS_L1CA,
            svid: 1,
            misc,
            code_lsb,
            doppler: Some(10_000),
            carrier_lsb,
            carrier_msb,
            cn0: None,
            lock_time: None,
            obs_info: MeasEpochObsInfo::empty(),
            n2: 0,
            channel_type2: Vec::new(),
        }
    }

    fn type2(
        offsets_msb: u8,
        code_offset_lsb: u16,
        doppler_offset_lsb: u16,
    ) -> MeasEpochChannelType2 {
        MeasEpochChannelType2 {
            type_field: GPS_L2C,
            lock_time: None,
            cn0: None,
            offsets_msb,
            carrier_msb: 0,
            obs_info: MeasEpochObsInfo::empty(),
            code_offset_lsb,
            carrier_lsb: 1_000,
            doppler_offset_lsb,
        }
    }

    // CodeMSB in bits 0-3 of misc supplies bits 32-35 of the pseudorange, and an all-zero code
    // marks it invalid.
    #[test]
    fn type1_pseudorange_and_doppler() {
        assert_eq!(type1(1, 0, 0, 0).pseudorange_m(), Some(4_294_967.296));
        assert_eq!(type1(0, 0, 0, 0).pseudorange_m(), None);
        assert_eq!(type1(0, 1_000, 0, 0).doppler_hz(), Some(1.0));
    }

    // The carrier phase adds the relative phase to the pseudorange in cycles, and CarrierMSB -128
    // with CarrierLSB 0 marks it invalid.
    #[test]
    fn type1_carrier_phase() {
        let expected = 20_000.0 * GPS_L1_HZ / SPEED_OF_LIGHT_M_S + 1.0;
        let phase = type1(0, 20_000_000, 0, 1_000)
            .carrier_phase_cycles()
            .unwrap();
        assert!((phase - expected).abs() < EPSILON);
        assert_eq!(type1(0, 20_000_000, -128, 0).carrier_phase_cycles(), None);
    }

    // OffsetsMSB 0xFF holds -1 in both the 3-bit code and the 5-bit Doppler fields, so the
    // offsets are -500 mm and -1 in units of 0.0001 Hz. A code offset MSB of -4 with LSB 0 and a
    // Doppler offset MSB of -16 with LSB 0 mark the offsets invalid.
    #[test]
    fn type2_offsets() {
        let parent = type1(0, 20_000_000, 0, 0);
        let offsets = type2(0xFF, 65_036, 65_535);
        let pseudorange = offsets.pseudorange_m(&parent).unwrap();
        assert!((pseudorange - 19_999.5).abs() < EPSILON);
        let doppler = offsets.doppler_hz(&parent).unwrap();
        assert!((doppler - (GPS_L2_HZ / GPS_L1_HZ - 0.0001)).abs() < EPSILON);
        assert_eq!(type2(0b100, 0, 1).pseudorange_m(&parent), None);
        assert_eq!(type2(0b1000_0000, 1, 0).doppler_hz(&parent), None);
    }
}
