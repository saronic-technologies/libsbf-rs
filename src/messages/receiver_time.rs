use crate::binrw_util;
use binrw::binrw;
use serde::Serialize;

// ReceiverTime Block 5914
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct ReceiverTime {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    /// UTC year, two digits from 0 to 99.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub utc_year: Option<i8>,
    /// UTC month, 1 to 12.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub utc_month: Option<i8>,
    /// UTC day of month, 1 to 31.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub utc_day: Option<i8>,
    /// UTC hour, 0 to 23.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub utc_hour: Option<i8>,
    /// UTC minute, 0 to 59.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub utc_min: Option<i8>,
    /// UTC second, 0 to 59.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub utc_sec: Option<i8>,
    /// Integer number of seconds GNSS time is ahead of UTC.
    #[br(map = binrw_util::map_i1)]
    #[bw(map = binrw_util::unmap_i1)]
    pub delta_ls: Option<i8>,
    /// Clock synchronization bit field.
    pub sync_level: u8,
}

impl ReceiverTime {
    /// Bit 0: the receiver week number is set.
    pub fn wn_set(&self) -> bool {
        self.sync_level & 1 != 0
    }

    /// Bit 1: the receiver time-of-week is set.
    pub fn tow_set(&self) -> bool {
        self.sync_level & (1 << 1) != 0
    }

    /// Bit 2: the time is set with sub-microsecond accuracy.
    pub fn fine_time(&self) -> bool {
        self.sync_level & (1 << 2) != 0
    }
}
