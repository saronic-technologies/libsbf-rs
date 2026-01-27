use binrw::binrw;

// AttCovEuler Block 5939
#[binrw]
#[derive(Debug)]
pub struct AttCovEuler {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub reserved: u8,
    pub error: u8,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_head_head: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_pitch_pitch: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_roll_roll: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_head_pitch: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_head_roll: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub cov_pitch_roll: Option<f32>,
}

impl AttCovEuler {
    // Error codes for baselines (bits 0-1 and 2-3)
    pub const ERROR_NO_ERROR: u8 = 0;
    pub const ERROR_NOT_ENOUGH_MEASUREMENTS: u8 = 1;

    // Bit 7 flag
    pub const ERROR_ATTITUDE_NOT_REQUESTED: u8 = 0x80;
}
