use crate::binrw_util;
use binrw::binrw;

use super::ext_sensor_status::ConnectionPort;

// IMU Setup Block 4224
#[binrw]
#[derive(Clone, Debug)]
pub struct ImuSetup {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    _reserved: u8,
    #[br(map = binrw_util::map_enum)]
    #[bw(map = binrw_util::unmap_enum)]
    pub serial_port: ConnectionPort,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub ant_lever_arm_x_m: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub ant_lever_arm_y_m: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub ant_lever_arm_z_m: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub theta_x_deg: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub theta_y_deg: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub theta_z_deg: Option<f32>,
}
