use crate::SubBlock;
use alloc::vec::Vec;
use binrw::binrw;

// External Sensor Measurement Block 4050
#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeas {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(crate::DO_NOT_USE_U4))]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) },
         map = |v: Vec<SubBlock<ExtSensorMeasSet>>| v.into_iter().map(SubBlock::into_inner).collect())]
    #[bw(args_raw = (usize::from(*sb_length),),
         map = |v: &Vec<ExtSensorMeasSet>| v.iter().cloned().map(SubBlock::from).collect::<Vec<_>>())]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct ExtSensorMeasAcceleration {
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub ax: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub ay: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub az: Option<f64>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasAngularRate {
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub wx: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub wy: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub wz: Option<f64>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasVelocity {
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub vx: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub vy: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub vz: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub stdx: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub stdy: Option<f32>,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f32>| x.unwrap_or(crate::DO_NOT_USE_F4))]
    pub stdz: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasInfo {
    #[br(map = |x: i16| if x == crate::DO_NOT_USE_I2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<i16>| x.unwrap_or(crate::DO_NOT_USE_I2))]
    pub sensor_temp: Option<i16>,
    pub _reserved: [u8; 22],
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasZeroVelocityFlag {
    pub zero_v_flag: f64,
    pub _reserved: [u8; 16],
}
