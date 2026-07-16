use crate::SubBlock;
use alloc::vec::Vec;
use binrw::binrw;

// DiskStatus Block 4059
#[binrw]
#[derive(Clone, Debug)]
pub struct DiskStatus {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(crate::DO_NOT_USE_U4))]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    pub reserved: [u8; 4],
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) },
         map = |v: Vec<SubBlock<DiskData>>| v.into_iter().map(SubBlock::into_inner).collect())]
    #[bw(args_raw = (usize::from(*sb_length),),
         map = |v: &Vec<DiskData>| v.iter().cloned().map(SubBlock::from).collect::<Vec<_>>())]
    pub disks: Vec<DiskData>,
}

// DiskData sub-block
#[binrw]
#[derive(Clone, Debug)]
pub struct DiskData {
    /// Disk identifier, starting at 1 for the internal SD card.
    pub disk_id: u8,
    /// Disk status bit field.
    pub status: u8,
    /// 16 most-significant bits of the disk usage in bytes.
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub disk_usage_msb: Option<u16>,
    /// 32 least-significant bits of the disk usage in bytes.
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(crate::DO_NOT_USE_U4))]
    pub disk_usage_lsb: Option<u32>,
    /// Total disk size in Mbytes.
    #[br(map = |x: u32| if x == 0 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(0))]
    pub disk_size: Option<u32>,
    /// Counter of file and folder create/delete events, wrapping at 255.
    pub create_delete_count: u8,
    /// Disk error code: 0 no error, 254 mount failed.
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u8>| x.unwrap_or(crate::DO_NOT_USE_U1))]
    pub error: Option<u8>,
}

impl DiskData {
    /// Total disk usage in bytes, combining the MSB and LSB halves.
    pub fn disk_usage_bytes(&self) -> Option<u64> {
        match (self.disk_usage_msb, self.disk_usage_lsb) {
            (Some(msb), Some(lsb)) => Some((u64::from(msb) << 32) | u64::from(lsb)),
            _ => None,
        }
    }

    /// Bit 0: the disk is mounted.
    pub fn mounted(&self) -> bool {
        self.status & 1 != 0
    }

    /// Bit 1: the disk is full, at least 95% used.
    pub fn full(&self) -> bool {
        self.status & (1 << 1) != 0
    }

    /// Bit 3: logging is enabled on the disk.
    pub fn logging_enabled(&self) -> bool {
        self.status & (1 << 3) != 0
    }
}
