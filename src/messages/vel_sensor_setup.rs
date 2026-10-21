use super::ext_sensor_status::ConnectionPort;
use crate::util::binrw::{
    map_f4, map_u1_enum, map_u2, map_u4, unmap_f4, unmap_u1_enum, unmap_u2, unmap_u4,
};
use alloc::vec::Vec;
use binrw::binrw;

// VelSensorSetup Block 4244
#[binrw]
#[derive(Clone, Debug)]
pub struct VelSensorSetup {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub reserved: u8,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub port: ConnectionPort,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub lever_arm_x: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub lever_arm_y: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub lever_arm_z: Option<f32>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
