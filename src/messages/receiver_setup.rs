use crate::binrw_util;
use binrw::binrw;
use serde::Serialize;

// Receiver Setup Block 5902
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct ReceiverSetup {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub reserved: [u8; 2],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub marker_name: [u8; 60],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub marker_number: [u8; 20],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub observer: [u8; 20],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub agency: [u8; 40],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub rx_serial_number: [u8; 20],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub rx_name: [u8; 20],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub rx_version: [u8; 20],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub ant_serial_nbr: [u8; 20],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub ant_type: [u8; 20],
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub delta_h: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub delta_e: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub delta_n: Option<f32>,
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub marker_type: [u8; 20],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub gnss_fw_version: [u8; 40],
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub product_name: [u8; 40],
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub latitude: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub longitude: Option<f64>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub height: Option<f32>,
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub station_code: [u8; 10],
    pub monument_idx: u8,
    pub receiver_idx: u8,
    #[serde(serialize_with = "crate::serde_util::ascii_bytes")]
    pub country_code: [u8; 3],
    pub reserved1: [u8; 21],
}
