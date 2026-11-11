use crate::util::{
    binrw::{
        map_u2, map_u4, subblock_count_mod256, unmap_u2, unmap_u4, unwrap_subblocks, wrap_subblocks,
    },
    signal::signal_number,
};
use alloc::vec::Vec;
use binrw::binrw;

// Step of the high-resolution C/N0 extension, in dB-Hz.
const CN0_HIGH_RES_STEP_DB_HZ: f32 = 0.03125;
// Step of the multipath and smoothing corrections of the pseudorange, in m.
const CODE_CORRECTION_STEP_M: f64 = 0.001;
// Step of the code tracking noise variance, in m².
const CODE_VAR_STEP_M2: f64 = 0.0001;
// Step of the carrier tracking noise variance, in cycles².
const CARRIER_VAR_STEP_CYCLES2: f64 = 1e-6;
// Step of the multipath correction of the carrier phase, in cycles.
const CARRIER_MP_CORR_STEP_CYCLES: f64 = 1.0 / 512.0;

// MeasExtra Block 4000
#[binrw]
#[derive(Clone, Debug)]
pub struct MeasExtra {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    pub doppler_var_factor: f32,
    #[br(temp, parse_with = subblock_count_mod256, args(n, sb_length))]
    #[bw(ignore)]
    count: usize,
    #[br(args { count, inner: (usize::from(sb_length),) }, map = unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = wrap_subblocks)]
    pub channel_sub: Vec<MeasExtraChannelSub>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct MeasExtraChannelSub {
    pub rx_channel: u8,
    pub type_field: u8,
    pub mp_correction: i16,
    pub smoothing_corr: i16,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub code_var: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub carrier_var: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub lock_time: Option<u16>,
    pub cum_loss_cont: u8,
    pub car_mp_corr: i8,
    pub info: u8,
    pub misc: u8,
}

impl MeasExtraChannelSub {
    /// Antenna ID from bits 5-7 of the type field: 0 main, 1 Aux1, 2 Aux2.
    pub fn antenna_id(&self) -> u8 {
        self.type_field >> 5
    }

    /// Signal number per section 4.1.10 of the reference guide: bits 0-4 of
    /// the type field, extended through bits 3-7 of misc when they read the
    /// escape value.
    pub fn signal_number(&self) -> u8 {
        signal_number(self.type_field, self.misc >> 3)
    }

    /// High-resolution extension of the MeasEpoch C/N0 from bits 0-2 of misc, from 0 to 7
    /// in units of 0.03125 dB-Hz. Adding it to the MeasEpoch C/N0 of the same signal gives
    /// the C/N0 at 0.03125 dB-Hz resolution.
    pub fn cn0_high_res(&self) -> u8 {
        self.misc & 0x07
    }

    /// C/N0 in dB-Hz at 0.03125 dB-Hz resolution, given the C/N0 in dB-Hz of the
    /// MeasEpoch sub-block for the same channel and signal.
    pub fn cn0_db_hz(&self, meas_epoch_cn0_db_hz: f32) -> f32 {
        meas_epoch_cn0_db_hz + f32::from(self.cn0_high_res()) * CN0_HIGH_RES_STEP_DB_HZ
    }

    /// Multipath correction applied to the pseudorange, in meters. Adding it to the
    /// pseudorange recovers the raw pseudorange as it would be if multipath mitigation was not
    /// used.
    pub fn mp_correction_m(&self) -> f64 {
        f64::from(self.mp_correction) * CODE_CORRECTION_STEP_M
    }

    /// Smoothing correction applied to the pseudorange, in meters. Adding it to the
    /// pseudorange recovers the raw pseudorange as it would be if smoothing was disabled.
    pub fn smoothing_corr_m(&self) -> f64 {
        f64::from(self.smoothing_corr) * CODE_CORRECTION_STEP_M
    }

    /// Estimated code tracking noise variance, in m², clipped at 6.5534 m².
    pub fn code_var_m2(&self) -> Option<f64> {
        self.code_var.map(|var| f64::from(var) * CODE_VAR_STEP_M2)
    }

    /// Estimated carrier tracking noise variance, in cycles², clipped at 0.065534 cycles².
    pub fn carrier_var_cycles2(&self) -> Option<f64> {
        self.carrier_var
            .map(|var| f64::from(var) * CARRIER_VAR_STEP_CYCLES2)
    }

    /// Multipath correction applied to the carrier phase, in cycles. Adding it to the carrier
    /// phase recovers the raw phase as it would be if multipath mitigation was not used.
    pub fn car_mp_corr_cycles(&self) -> f64 {
        f64::from(self.car_mp_corr) * CARRIER_MP_CORR_STEP_CYCLES
    }
}

#[cfg(test)]
mod tests {
    use super::MeasExtra;
    use alloc::vec::Vec;
    use binrw::{io::Cursor, BinRead, BinWrite};

    // N holds 300 modulo 256, so all 300 sub-blocks can be found only through the
    // body length. Each sub-block carries its index as the multipath correction.
    #[test]
    fn reads_more_than_255_sub_blocks() {
        let body: Vec<u8> = 1000u32
            .to_le_bytes()
            .into_iter()
            .chain(2400u16.to_le_bytes())
            .chain([44, 16])
            .chain(1.0f32.to_le_bytes())
            .chain((0..300u16).flat_map(|i| {
                [(i % 256) as u8, 0]
                    .into_iter()
                    .chain((i as i16).to_le_bytes())
                    .chain([0; 12])
            }))
            .collect();

        let block = MeasExtra::read_le(&mut Cursor::new(&body)).unwrap();
        assert_eq!(block.n, 44);
        let corrections: Vec<i16> = block
            .channel_sub
            .iter()
            .map(|sub| sub.mp_correction)
            .collect();
        assert_eq!(corrections, (0..300).collect::<Vec<i16>>());

        let mut out = Vec::new();
        block.write_le(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(out, body);
    }
}
