use binrw::binrw;
use crate::do_not_use::{map_u2, map_u4, map_f4, map_f8, unmap_u2, unmap_u4, unmap_f4, unmap_f8};

// Receiver Setup Block 5902
#[binrw]
#[derive(Debug, Clone)]
pub struct ReceiverSetup {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub reserved: [u8; 2],
    pub marker_name: [u8; 60],
    pub marker_number: [u8; 20],
    pub observer: [u8; 20],
    pub agency: [u8; 40],
    pub rx_serial_number: [u8; 20],
    pub rx_name: [u8; 20],
    pub rx_version: [u8; 20],
    pub ant_serial_nbr: [u8; 20],
    pub ant_type: [u8; 20],
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_h: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_e: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub delta_n: Option<f32>,
    pub marker_type: [u8; 20],
    pub gnss_fw_version: [u8; 40],
    pub product_name: [u8; 40],
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub latitude: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub longitude: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub height: Option<f32>,
    pub station_code: [u8; 10],
    pub monument_idx: u8,
    pub receiver_idx: u8,
    pub country_code: [u8; 3],
    pub reserved1: [u8; 21],
}