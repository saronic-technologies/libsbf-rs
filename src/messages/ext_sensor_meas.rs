use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

use super::ext_sensor_status::{ConnectionPort, ExtSensorModel};

// External Sensor Measurement Block 4050
#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeas {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = binrw_util::unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = binrw_util::wrap_subblocks)]
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
    #[br(map = |x: u8| ConnectionPort::from(x))]
    #[bw(map = |x: &ConnectionPort| u8::from(*x))]
    pub source: ConnectionPort,
    #[br(map = |x: u8| ExtSensorModel::from(x))]
    #[bw(map = |x: &ExtSensorModel| *x as u8)]
    pub sensor_model: ExtSensorModel,
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
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub ax: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub ay: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub az: Option<f64>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasAngularRate {
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub wx: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub wy: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub wz: Option<f64>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasVelocity {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vx: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vy: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vz: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub stdx: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub stdy: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub stdz: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasInfo {
    #[br(map = binrw_util::map_i2)]
    #[bw(map = binrw_util::unmap_i2)]
    pub sensor_temp: Option<i16>,
    pub _reserved: [u8; 22],
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorMeasZeroVelocityFlag {
    pub zero_v_flag: f64,
    pub _reserved: [u8; 16],
}
