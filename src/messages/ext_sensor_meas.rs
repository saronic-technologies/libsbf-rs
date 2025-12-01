use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_i2, map_u2, map_u4, map_f4, map_f8, unmap_i2, unmap_u2, unmap_u4, unmap_f4, unmap_f8};

// External Sensor Measurement Block 4050
#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeas {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(count = n as usize)]
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
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub ax: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub ay: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub az: Option<f64>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasAngularRate {
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub wx: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub wy: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub wz: Option<f64>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasVelocity {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vz: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub stdx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub stdy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub stdz: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasInfo {
    #[br(map = map_i2)]
    #[bw(map = unmap_i2)]
    pub sensor_temp: Option<i16>,
    pub _reserved: [u8; 22],
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorMeasZeroVelocityFlag {
    pub zero_v_flag: f64,
    pub _reserved: [u8; 16],
}