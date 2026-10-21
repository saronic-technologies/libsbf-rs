use crate::util::binrw::{map_u2, map_u4, map_u4_zero, unmap_u2, unmap_u4, unmap_u4_zero};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// RxMessage Block 4103
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct RxMessage {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Message type: 1 command reply, 2 logging, 3 FTP, 4 status, 5 slave GNSS, 6 CloudIt.
    pub message_type: u8,
    /// Severity: 1 info, 2 warning, 3 error.
    pub severity: u8,
    /// Unique message counter, starting at 1.
    #[br(map = map_u4_zero)]
    #[bw(map = unmap_u4_zero)]
    pub message_id: Option<u32>,
    /// Length of `message` in bytes, including the terminating NUL.
    pub string_ln: u16,
    pub reserved2: [u8; 2],
    #[br(count = usize::from(string_ln))]
    pub message: Vec<u8>,
}
