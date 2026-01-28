use alloc::vec::Vec;
use binrw::binrw;

// External Sensor Measurement Block 4050
#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeas {
    #[br(map = |x| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(count = usize::from(n))]
    pub ext_sensor_meas_set: Vec<ExtSensorMeasSet>,
}

#[repr(u8)]
pub enum ExtSensorMeasSetType {
    Acceleration = 0,
    AngularRate = 1,
    _Reserved = 2,
    Info = 3,
    Velocity = 4,
    ZeroVelocityFlag = 20,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasSet {
    pub source: u8,
    pub sensor_model: u8,
    pub type_: u8,
    pub obs_info: u8,

    #[br(if(type_ == ExtSensorMeasSetType::Acceleration as u8))]
    pub acc: Option<ExtSensorMeasAcceleration>,
    #[br(if(type_ == ExtSensorMeasSetType::AngularRate as u8))]
    pub ang_rate: Option<ExtSensorMeasAngularRate>,
    #[br(if(type_ == ExtSensorMeasSetType::Info as u8))]
    pub info: Option<ExtSensorMeasInfo>,
    #[br(if(type_ == ExtSensorMeasSetType::Velocity as u8))]
    pub vel: Option<ExtSensorMeasVelocity>,
    #[br(if(type_ == ExtSensorMeasSetType::ZeroVelocityFlag as u8))]
    pub zero_vel_flag: Option<ExtSensorMeasZeroVelocityFlag>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasAcceleration {
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub ax: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub ay: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub az: Option<f64>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasAngularRate {
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub wx: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub wy: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub wz: Option<f64>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasVelocity {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vx: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vy: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vz: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub stdx: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub stdy: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub stdz: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasInfo {
    #[br(map = |x| if x == crate::DO_NOT_USE_I2 { None } else { Some(x) })]
    pub sensor_temp: Option<i16>,
    pub _reserved: [u8; 22],
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasZeroVelocityFlag {
    pub zero_v_flag: f64,
    pub _reserved: [u8; 16],
}
