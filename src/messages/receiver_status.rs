use alloc::vec::Vec;
use binrw::BinRead;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ExtError: u8 {
        const SIS = 0x01;
        const DIFF_CORR = 0x02;
        const EXT_SENSOR = 0x04;
        const SETUP = 0x08;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RxState: u32 {
        const ACTIVE_ANTENNA = 1 << 1;
        const EXT_FREQ = 1 << 2;
        const EXT_TIME = 1 << 3;
        const WN_SET = 1 << 4;
        const TOW_SET = 1 << 5;
        const FINE_TIME = 1 << 6;
        const INTERNAL_DISK_ACTIVITY = 1 << 7;
        const INTERNAL_DISK_FULL = 1 << 8;
        const INTERNAL_DISK_MOUNTED = 1 << 9;
        const INT_ANT = 1 << 10;
        const REFOUT_LOCKED = 1 << 11;
        const EXTERNAL_DISK_ACTIVITY = 1 << 13;
        const EXTERNAL_DISK_FULL = 1 << 14;
        const EXTERNAL_DISK_MOUNTED = 1 << 15;
        const PPS_IN_CAL = 1 << 16;
        const DIFFCORR_IN = 1 << 17;
        const INTERNET = 1 << 18;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RxError: u32 {
        const SOFTWARE = 1 << 3;
        const WATCHDOG = 1 << 4;
        const ANTENNA = 1 << 5;
        const CONGESTION = 1 << 6;
        const MISSED_EVENT = 1 << 8;
        const CPU_OVERLOAD = 1 << 9;
        const INVALID_CONFIG = 1 << 10;
        const OUT_OF_GEOFENCE = 1 << 11;
    }
}

// ReceiverStatus Block 4014
#[derive(Debug, BinRead)]
pub struct ReceiverStatus {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub cpu_load: Option<u8>,
    #[br(map = |x: u8| ExtError::from_bits_truncate(x))]
    pub ext_error: ExtError,
    pub up_time: u32,
    #[br(map = |x: u32| RxState::from_bits_truncate(x))]
    pub rx_state: RxState,
    #[br(map = |x: u32| RxError::from_bits_truncate(x))]
    pub rx_error: RxError,
    pub n: u8,
    pub sb_length: u8,
    pub cmd_count: u8,
    pub temperature: u8,
    #[br(count = usize::from(n))]
    pub agc_state: Vec<AGCState>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

#[derive(Debug, BinRead)]
pub struct AGCState {
    pub frontend_id: u8,
    #[br(map = |x: i8| if x == -128 { None } else { Some(x) })]
    pub gain: Option<i8>,
    pub sample_var: u8,
    pub blanking_stat: u8,
}

impl AGCState {
    pub fn frontend_code(&self) -> u8 {
        self.frontend_id & 0x1F
    }

    pub fn antenna_id(&self) -> u8 {
        (self.frontend_id >> 5) & 0x07
    }
}
