use alloc::vec::Vec;
use binrw::BinRead;

// RxMessage Block 4103
#[derive(BinRead, Clone, Debug)]
pub struct RxMessage {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    /// Message type: 1 command reply, 2 logging, 3 FTP, 4 status, 5 slave GNSS, 6 CloudIt.
    pub message_type: u8,
    /// Severity: 1 info, 2 warning, 3 error.
    pub severity: u8,
    /// Unique message counter, starting at 1.
    #[br(map = |x: u32| if x == 0 { None } else { Some(x) })]
    pub message_id: Option<u32>,
    /// Length of `message` in bytes, including the terminating NUL.
    pub string_ln: u16,
    pub reserved2: [u8; 2],
    #[br(count = usize::from(string_ln))]
    pub message: Vec<u8>,
}
