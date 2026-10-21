use super::common::{ConnectionPort, ExtSensorModel};
use crate::util::binrw::{map_u1_enum, map_u2, map_u4, unmap_u1_enum, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// ExtSensorInfo Block 4222
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct ExtSensorInfo {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub source: ConnectionPort,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
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
