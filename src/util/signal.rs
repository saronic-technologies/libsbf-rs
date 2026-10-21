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

#[cfg(test)]
mod tests {
    use super::{cn0_db_hz, signal_number};

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
}
