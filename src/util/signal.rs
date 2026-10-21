// SigIdxLo value in a type field that extends the signal number into a second field.
const SIG_NR_ESCAPE: u8 = 31;
// Base added to the extension bits to form the signal number.
const SIG_NR_EXTENSION_OFFSET: u8 = 32;
// Step of a raw MeasEpoch C/N0 value, in dB-Hz.
const CN0_STEP_DB_HZ: f32 = 0.25;
// Offset added to the C/N0 of every signal except those in CN0_NO_OFFSET_SIG_NRS, in dB-Hz.
const CN0_OFFSET_DB_HZ: f32 = 10.0;
// Signal numbers whose C/N0 has no offset: GPS L1P and L2P.
const CN0_NO_OFFSET_SIG_NRS: [u8; 2] = [1, 2];
// Offset of a GLONASS frequency number as SBF stores it.
const GLONASS_FREQ_NR_OFFSET: i8 = 8;
// Carrier frequencies from section 4.1.10 of the reference guide, in Hz.
const L1_HZ: f64 = 1_575.42e6;
const L2_HZ: f64 = 1_227.60e6;
const L5_HZ: f64 = 1_176.45e6;
const E5B_HZ: f64 = 1_207.14e6;
const E5_ALTBOC_HZ: f64 = 1_191.795e6;
const E6_HZ: f64 = 1_278.75e6;
const B1I_HZ: f64 = 1_561.098e6;
const B3I_HZ: f64 = 1_268.52e6;
const GLONASS_L3_HZ: f64 = 1_202.025e6;
// GLONASS FDMA carrier frequency at frequency number 0, and its step per frequency number, in Hz.
const GLONASS_L1_BASE_HZ: f64 = 1_602.0e6;
const GLONASS_L1_STEP_HZ: f64 = 0.5625e6;
const GLONASS_L2_BASE_HZ: f64 = 1_246.0e6;
const GLONASS_L2_STEP_HZ: f64 = 0.4375e6;
// Signal numbers on each carrier frequency.
const L1_SIG_NRS: [u8; 11] = [0, 1, 5, 6, 13, 17, 24, 32, 33, 37, 38];
const L2_SIG_NRS: [u8; 3] = [2, 3, 7];
const L5_SIG_NRS: [u8; 7] = [4, 14, 15, 20, 25, 26, 39];
const E5B_SIG_NRS: [u8; 3] = [21, 29, 34];
const E5_ALTBOC_SIG_NR: u8 = 22;
const E6_SIG_NRS: [u8; 2] = [19, 27];
const B1I_SIG_NR: u8 = 28;
const B3I_SIG_NR: u8 = 30;
const GLONASS_L1_SIG_NRS: [u8; 2] = [8, 9];
const GLONASS_L2_SIG_NRS: [u8; 2] = [10, 11];
const GLONASS_L3_SIG_NR: u8 = 12;

/// Signal number per section 4.1.10 of the reference guide, from bits 0-4 of a
/// type field. When those bits read the escape value 31, the signal number is
/// 32 plus `extension`, the extension bits a block carries in a second field.
pub(crate) fn signal_number(type_field: u8, extension: u8) -> u8 {
    let sig_idx_lo = type_field & 0x1F;
    if sig_idx_lo == SIG_NR_ESCAPE {
        SIG_NR_EXTENSION_OFFSET + extension
    } else {
        sig_idx_lo
    }
}

/// C/N0 in dB-Hz of a signal with the given signal number, from a raw MeasEpoch
/// C/N0 value: 0.25 dB-Hz steps, plus 10 dB-Hz except for signal numbers 1 and 2.
pub(crate) fn cn0_db_hz(cn0: u8, signal_number: u8) -> f32 {
    let cn0 = f32::from(cn0) * CN0_STEP_DB_HZ;
    if CN0_NO_OFFSET_SIG_NRS.contains(&signal_number) {
        cn0
    } else {
        cn0 + CN0_OFFSET_DB_HZ
    }
}

/// GLONASS frequency number, from a value SBF stores with an offset of 8. GLONASS reduced
/// its channel plan from frequency numbers 0 to 24 to -7 to 6 for all satellites launched
/// after 2005, per section 3.3.1.1 of the [GLONASS L1, L2 ICD, edition 5.1][icd]. Values up
/// to 13 from before that are supported in firmware prior to 1.5.x. [GPS World][gps-world]
/// indicates that the reduction happened to preserve radio astronomy bands.
///
/// [icd]: https://www.unavco.org/help/glossary/docs/ICD_GLONASS_5.1_(2008)_en.pdf#page=15
/// [gps-world]: https://www.gpsworld.com/glonass-the-decade-of-transition-to-cdma-signals/
pub(crate) fn glonass_frequency_number(stored: u8) -> i8 {
    stored as i8 - GLONASS_FREQ_NR_OFFSET
}

/// Carrier frequency in Hz of a signal per section 4.1.10 of the reference guide. The GLONASS
/// L1 and L2 FDMA signals need the satellite's frequency number. The result is None for a
/// signal number the guide does not define, for the L-band signal, whose frequency is beam
/// specific, and for a GLONASS FDMA signal without a frequency number.
pub(crate) fn carrier_frequency_hz(signal_number: u8, glonass_freq_nr: Option<i8>) -> Option<f64> {
    match signal_number {
        n if L1_SIG_NRS.contains(&n) => Some(L1_HZ),
        n if L2_SIG_NRS.contains(&n) => Some(L2_HZ),
        n if L5_SIG_NRS.contains(&n) => Some(L5_HZ),
        n if E5B_SIG_NRS.contains(&n) => Some(E5B_HZ),
        n if E6_SIG_NRS.contains(&n) => Some(E6_HZ),
        E5_ALTBOC_SIG_NR => Some(E5_ALTBOC_HZ),
        B1I_SIG_NR => Some(B1I_HZ),
        B3I_SIG_NR => Some(B3I_HZ),
        GLONASS_L3_SIG_NR => Some(GLONASS_L3_HZ),
        n if GLONASS_L1_SIG_NRS.contains(&n) => {
            glonass_freq_nr.map(|k| GLONASS_L1_BASE_HZ + f64::from(k) * GLONASS_L1_STEP_HZ)
        }
        n if GLONASS_L2_SIG_NRS.contains(&n) => {
            glonass_freq_nr.map(|k| GLONASS_L2_BASE_HZ + f64::from(k) * GLONASS_L2_STEP_HZ)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{carrier_frequency_hz, cn0_db_hz, signal_number};

    // Signal numbers 1 and 2 are GPS L1P and L2P, which carry no 10 dB-Hz offset.
    #[test]
    fn cn0_offset_depends_on_signal_number() {
        assert_eq!(cn0_db_hz(160, 0), 50.0);
        assert_eq!(cn0_db_hz(160, 1), 40.0);
        assert_eq!(cn0_db_hz(160, 2), 40.0);
        assert_eq!(cn0_db_hz(160, 17), 50.0);
    }

    // SigIdxLo 31 takes the signal number from the extension bits instead.
    #[test]
    fn signal_number_escape_uses_extension() {
        assert_eq!(signal_number(0b001_00011, 5), 3);
        assert_eq!(signal_number(0b010_11111, 5), 37);
    }

    // GPS L1CA has a fixed frequency, GLONASS L1CA shifts by 9/16 MHz per frequency number, and
    // the L-band signal and undefined signal numbers have no frequency.
    #[test]
    fn carrier_frequency_by_signal() {
        assert_eq!(carrier_frequency_hz(0, None), Some(1_575.42e6));
        assert_eq!(carrier_frequency_hz(8, Some(-7)), Some(1_598.062_5e6));
        assert_eq!(carrier_frequency_hz(10, Some(6)), Some(1_248.625e6));
        assert_eq!(carrier_frequency_hz(8, None), None);
        assert_eq!(carrier_frequency_hz(23, None), None);
        assert_eq!(carrier_frequency_hz(16, None), None);
    }
}
