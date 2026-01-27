use binrw::binrw;

// IMU Setup Block 4224
#[binrw]
#[derive(Debug)]
pub struct ImuSetup {
    #[br(map = |x| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    _reserved: u8,
    // TODO: create SerialPort enum for future serial port info
    pub serial_port: u8,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ant_lever_arm_x_m: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ant_lever_arm_y_m: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ant_lever_arm_z_m: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub theta_x_deg: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub theta_y_deg: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub theta_z_deg: Option<f32>,
}
