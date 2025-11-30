use binrw::binrw;
use alloc::vec::Vec;

// ReceiverStatus Block 4014
#[binrw]
#[derive(Debug, Clone)]
pub struct ReceiverStatus {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    #[br(map = crate::do_not_use::map_u1)]
    #[bw(map = |x| crate::do_not_use::unmap_u1(x))]
    pub cpu_load: Option<u8>,
    pub ext_error: u8,
    pub up_time: u32,
    pub rx_state: u32,
    pub rx_error: u32,
    pub n: u8,
    pub sb_length: u8,
    pub cmd_count: u8,
    pub temperature: u8,
    #[br(count = n)]
    pub agc_state: Vec<AGCState>,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = crate::do_not_use::write_vec)]
    pub padding: Vec<u8>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct AGCState {
    pub frontend_id: u8,
    #[br(map = |x: i8| if x == -128 { None } else { Some(x) })]
    pub gain: Option<i8>,
    pub sample_var: u8,
    pub blanking_stat: u8,
    // Note: padding handled by parent based on sb_length
}

impl ReceiverStatus {
    // External Error bits
    pub const EXT_ERROR_SIS: u8 = 0x01;
    pub const EXT_ERROR_DIFFCORR: u8 = 0x02;
    pub const EXT_ERROR_EXTSENSOR: u8 = 0x04;
    pub const EXT_ERROR_SETUP: u8 = 0x08;
    
    // RxState bits
    pub const STATE_ACTIVE_ANTENNA: u32 = 0x00000002;
    pub const STATE_EXT_FREQ: u32 = 0x00000004;
    pub const STATE_EXT_TIME: u32 = 0x00000008;
    pub const STATE_WN_SET: u32 = 0x00000010;
    pub const STATE_TOW_SET: u32 = 0x00000020;
    pub const STATE_FINE_TIME: u32 = 0x00000040;
    pub const STATE_INTERNAL_DISK_ACTIVITY: u32 = 0x00000080;
    pub const STATE_INTERNAL_DISK_FULL: u32 = 0x00000100;
    pub const STATE_INTERNAL_DISK_MOUNTED: u32 = 0x00000200;
    pub const STATE_INT_ANT: u32 = 0x00000400;
    pub const STATE_REFOUT_LOCKED: u32 = 0x00000800;
    pub const STATE_EXTERNAL_DISK_ACTIVITY: u32 = 0x00002000;
    pub const STATE_EXTERNAL_DISK_FULL: u32 = 0x00004000;
    pub const STATE_EXTERNAL_DISK_MOUNTED: u32 = 0x00008000;
    pub const STATE_PPS_IN_CAL: u32 = 0x00010000;
    pub const STATE_DIFFCORR_IN: u32 = 0x00020000;
    pub const STATE_INTERNET: u32 = 0x00040000;
    
    // RxError bits
    pub const ERROR_SOFTWARE: u32 = 0x00000008;
    pub const ERROR_WATCHDOG: u32 = 0x00000010;
    pub const ERROR_ANTENNA: u32 = 0x00000020;
    pub const ERROR_CONGESTION: u32 = 0x00000040;
    pub const ERROR_MISSED_EVENT: u32 = 0x00000100;
    pub const ERROR_CPU_OVERLOAD: u32 = 0x00000200;
    pub const ERROR_INVALID_CONFIG: u32 = 0x00000400;
    pub const ERROR_OUT_OF_GEOFENCE: u32 = 0x00000800;
}

impl AGCState {
    // Frontend codes (bits 0-4)
    pub const FRONTEND_GPS_L1_E1: u8 = 0;
    pub const FRONTEND_GLO_L1: u8 = 1;
    pub const FRONTEND_E6: u8 = 2;
    pub const FRONTEND_GPS_L2: u8 = 3;
    pub const FRONTEND_GLO_L2: u8 = 4;
    pub const FRONTEND_L5_E5A_B2A: u8 = 5;
    pub const FRONTEND_E5B_B2B: u8 = 6;
    pub const FRONTEND_E5_AB: u8 = 7;
    pub const FRONTEND_COMBINED_L1: u8 = 8;
    pub const FRONTEND_COMBINED_L2: u8 = 9;
    pub const FRONTEND_MSS_LBAND: u8 = 10;
    pub const FRONTEND_B1: u8 = 11;
    pub const FRONTEND_B3: u8 = 12;
    pub const FRONTEND_S_BAND: u8 = 13;
    pub const FRONTEND_B3_E6: u8 = 14;
    
    pub fn frontend_code(&self) -> u8 {
        self.frontend_id & 0x1F
    }
    
    pub fn antenna_id(&self) -> u8 {
        (self.frontend_id >> 5) & 0x07
    }
}