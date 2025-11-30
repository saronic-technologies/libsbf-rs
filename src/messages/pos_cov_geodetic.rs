use binrw::binrw;

// PosCovGeodetic Block 5906
#[binrw]
#[derive(Debug, Clone)]
pub struct PosCovGeodetic {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub mode: u8,
    pub error: u8,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_latlat: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_lonlon: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_hgthgt: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_bb: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_latlon: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_lathgt: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_latb: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_lonhgt: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_lonb: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_hb: Option<f32>,
}

impl PosCovGeodetic {
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
}