use binrw::binrw;
use alloc::vec::Vec;

// VelSensorSetup Block 4244
#[binrw]
#[derive(Debug)]
pub struct VelSensorSetup {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub reserved: u8,
    pub port: u8,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub lever_arm_x: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub lever_arm_y: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub lever_arm_z: Option<f32>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl VelSensorSetup {
    // Port constants
    pub const PORT_COM1: u8 = 0;
    pub const PORT_COM2: u8 = 1;
    pub const PORT_COM3: u8 = 2;
    pub const PORT_COM4: u8 = 3;
    pub const PORT_USB1: u8 = 5;
    pub const PORT_USB2: u8 = 6;
    pub const PORT_IP10: u8 = 7;
    pub const PORT_IP11: u8 = 8;
    pub const PORT_IP12: u8 = 9;
    pub const PORT_IP13: u8 = 10;
    pub const PORT_IP14: u8 = 11;
    pub const PORT_IP15: u8 = 12;
    pub const PORT_IP16: u8 = 13;
    pub const PORT_IP17: u8 = 14;
    pub const PORT_IPS1: u8 = 15;
    pub const PORT_IPS2: u8 = 16;
    pub const PORT_IPS3: u8 = 17;
    pub const PORT_IPS4: u8 = 18;
    pub const PORT_IPS5: u8 = 19;
    pub const PORT_IPR1: u8 = 20;
    pub const PORT_IPR2: u8 = 21;
    pub const PORT_IPR3: u8 = 22;
    pub const PORT_IPR4: u8 = 23;
    pub const PORT_IPR5: u8 = 24;
    pub const PORT_INTERNAL_IMU: u8 = 32;
}