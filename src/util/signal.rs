// SigIdxLo value in a type field that extends the signal number into a second field.
const SIG_NR_ESCAPE: u8 = 31;
// Base added to the extension bits to form the signal number.
const SIG_NR_EXTENSION_OFFSET: u8 = 32;

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
