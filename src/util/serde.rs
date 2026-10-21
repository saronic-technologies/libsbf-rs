//! Serde helpers for wire types without a usable derived form.

use alloc::string::String;
use bitflags::Flags;
use serde::{Serialize, Serializer};

/// Serializes a bitflags value as its raw integer word, since the bitflags
/// `serde` feature renders flag-name strings where consumers want the
/// number.
pub(crate) fn flag_bits<S, F>(flags: &F, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    F: Flags,
    F::Bits: Serialize,
{
    flags.bits().serialize(serializer)
}

/// Serializes a fixed-size ASCII field as a string with trailing NULs
/// dropped, since serde derives no impls for arrays past 32 elements and a
/// number list hides the text.
pub(crate) fn ascii_bytes<S, const N: usize>(
    bytes: &[u8; N],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(String::from_utf8_lossy(bytes).trim_end_matches('\0'))
}
