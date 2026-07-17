use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

use super::ext_sensor_status::{ConnectionPort, ExtSensorModel};

// ExtSensorInfo Block 4222
#[binrw]
#[derive(Clone, Debug)]
pub struct ExtSensorInfo {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = |x: u8| ConnectionPort::from(x))]
    #[bw(map = |x: &ConnectionPort| u8::from(*x))]
    pub source: ConnectionPort,
    #[br(map = |x: u8| ExtSensorModel::from(x))]
    #[bw(map = |x: &ExtSensorModel| *x as u8)]
    pub sensor_model: ExtSensorModel,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub data: Vec<u8>,
}

impl ExtSensorInfo {
    // Data structure sizes
    pub const SBG_DATA_SIZE: usize = 52;
    pub const VN100_DATA_SIZE: usize = 36;
    pub const ADIS1650X_DATA_SIZE: usize = 44;
}
