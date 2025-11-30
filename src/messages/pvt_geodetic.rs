use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_u1, map_u2, map_u4, map_f4, map_f8, unmap_u1, unmap_u2, unmap_u4, unmap_f4, unmap_f8, write_vec};

// PVTGeodetic Block 4007
#[binrw]
#[derive(Debug, Clone)]
pub struct PVTGeodetic {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub mode: u8,
    pub error: u8,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub latitude: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub longitude: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub height: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub undulation: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vn: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ve: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vu: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cog: Option<f32>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub rx_clk_bias: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub rx_clk_drift: Option<f32>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub time_system: Option<u8>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub datum: Option<u8>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nr_sv: Option<u8>,
    pub wa_corr_info: u8,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub reference_id: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub mean_corr_age: Option<u16>,
    pub signal_info: u32,
    pub alert_flag: u8,
    // Rev 1 fields
    pub nr_bases: u8,
    pub ppp_info: u16,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub latency: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub h_accuracy: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub v_accuracy: Option<u16>,
    pub misc: u8,
    // Rev 2 fields
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = write_vec)]
    pub padding: Vec<u8>,
}

impl PVTGeodetic {
    // Mode bits 0-3: PVT solution type
    pub const MODE_NO_PVT: u8 = 0;
    pub const MODE_STANDALONE: u8 = 1;
    pub const MODE_DIFFERENTIAL: u8 = 2;
    pub const MODE_FIXED: u8 = 3;
    pub const MODE_RTK_FIXED: u8 = 4;
    pub const MODE_RTK_FLOAT: u8 = 5;
    pub const MODE_SBAS: u8 = 6;
    pub const MODE_MOVING_BASE_RTK_FIXED: u8 = 7;
    pub const MODE_MOVING_BASE_RTK_FLOAT: u8 = 8;
    pub const MODE_PPP: u8 = 10;
    
    // Error codes
    pub const ERROR_NONE: u8 = 0;
    pub const ERROR_NOT_ENOUGH_MEAS: u8 = 1;
    pub const ERROR_NOT_ENOUGH_EPH: u8 = 2;
    pub const ERROR_DOP_TOO_LARGE: u8 = 3;
    pub const ERROR_RESIDUALS_TOO_LARGE: u8 = 4;
    pub const ERROR_NO_CONVERGENCE: u8 = 5;
    pub const ERROR_NOT_ENOUGH_AFTER_OUTLIER: u8 = 6;
    pub const ERROR_POSITION_PROHIBITED: u8 = 7;
    pub const ERROR_NOT_ENOUGH_DIFF_CORR: u8 = 8;
    pub const ERROR_BASE_COORDS_UNAVAILABLE: u8 = 9;
    pub const ERROR_AMBIGUITIES_NOT_FIXED: u8 = 10;
    
    // Time system values
    pub const TIME_GPS: u8 = 0;
    pub const TIME_GALILEO: u8 = 1;
    pub const TIME_GLONASS: u8 = 3;
    pub const TIME_BEIDOU: u8 = 4;
    pub const TIME_QZSS: u8 = 5;
    pub const TIME_FUGRO: u8 = 100;
    
    // Datum values
    pub const DATUM_WGS84: u8 = 0;
    pub const DATUM_BASE_STATION: u8 = 19;
    pub const DATUM_ETRS89: u8 = 30;
    pub const DATUM_NAD83_2011: u8 = 31;
    pub const DATUM_NAD83_PA11: u8 = 32;
    pub const DATUM_NAD83_MA11: u8 = 33;
    pub const DATUM_GDA94_2010: u8 = 34;
    pub const DATUM_GDA2020: u8 = 35;
    pub const DATUM_JGD2011: u8 = 36;
    pub const DATUM_USER_1: u8 = 250;
    pub const DATUM_USER_2: u8 = 251;
    
    // RAIM integrity flags (bits 0-1 of alert_flag)
    pub const RAIM_NOT_ACTIVE: u8 = 0;
    pub const RAIM_TEST_SUCCESS: u8 = 1;
    pub const RAIM_TEST_FAILED: u8 = 2;
}