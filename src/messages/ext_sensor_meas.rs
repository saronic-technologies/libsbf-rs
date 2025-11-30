use binrw::binrw;
use alloc::vec::Vec;

// External Sensor Measurement Block 4050
#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeas {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(count = n)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ExtSensorMeasAcceleration {
    #[br(map = crate::do_not_use::map_f8)]
    #[bw(map = |x| crate::do_not_use::unmap_f8(x))]
    pub ax: Option<f64>,
    #[br(map = crate::do_not_use::map_f8)]
    #[bw(map = |x| crate::do_not_use::unmap_f8(x))]
    pub ay: Option<f64>,
    #[br(map = crate::do_not_use::map_f8)]
    #[bw(map = |x| crate::do_not_use::unmap_f8(x))]
    pub az: Option<f64>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasAngularRate {
    #[br(map = crate::do_not_use::map_f8)]
    #[bw(map = |x| crate::do_not_use::unmap_f8(x))]
    pub wx: Option<f64>,
    #[br(map = crate::do_not_use::map_f8)]
    #[bw(map = |x| crate::do_not_use::unmap_f8(x))]
    pub wy: Option<f64>,
    #[br(map = crate::do_not_use::map_f8)]
    #[bw(map = |x| crate::do_not_use::unmap_f8(x))]
    pub wz: Option<f64>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasVelocity {
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub vx: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub vy: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub vz: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub stdx: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub stdy: Option<f32>,
    #[br(map = crate::do_not_use::map_f4)]
    #[bw(map = |x| crate::do_not_use::unmap_f4(x))]
    pub stdz: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasInfo {
    #[br(map = crate::do_not_use::map_i2)]
    #[bw(map = |x| crate::do_not_use::unmap_i2(x))]
    pub sensor_temp: Option<i16>,
    pub _reserved: [u8; 22],
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasZeroVelocityFlag {
    pub zero_v_flag: f64,
    pub _reserved: [u8; 16],
}