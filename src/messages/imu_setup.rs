use binrw::binrw;
use crate::do_not_use::{map_u2, map_u4, map_f4, unmap_u2, unmap_u4, unmap_f4};

// IMU Setup Block 4224
#[binrw]
#[derive(Debug, Clone)]
pub struct ImuSetup {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    _reserved: u8,
    // TODO: create SerialPort enum for future serial port info
    pub serial_port: u8,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ant_lever_arm_x_m: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ant_lever_arm_y_m: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ant_lever_arm_z_m: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub theta_x_deg: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub theta_y_deg: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub theta_z_deg: Option<f32>,
}