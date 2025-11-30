use binrw::binrw;

// AttCovEuler Block 5939
#[binrw]
#[derive(Debug, Clone)]
pub struct AttCovEuler {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub reserved: u8,
    pub error: u8,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_head_head: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_pitch_pitch: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_roll_roll: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_head_pitch: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_head_roll: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub cov_pitch_roll: Option<f32>,
}

impl AttCovEuler {
    // Error codes for baselines (bits 0-1 and 2-3)
    pub const ERROR_NO_ERROR: u8 = 0;
    pub const ERROR_NOT_ENOUGH_MEASUREMENTS: u8 = 1;
    
    // Bit 7 flag
    pub const ERROR_ATTITUDE_NOT_REQUESTED: u8 = 0x80;
}