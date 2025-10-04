use binrw::binrw;

// Receiver Setup Block 5902
#[binrw]
#[derive(Debug)]
pub struct ReceiverSetup {
    #[br(map = |x| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
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
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_h: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_e: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_n: Option<f32>,
    pub marker_type: [u8; 20],
    pub gnss_fw_version: [u8; 40],
    pub product_name: [u8; 40],
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub latitude: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub longitude: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub height: Option<f32>,
    pub station_code: [u8; 10],
    pub monument_idx: u8,
    pub receiver_idx: u8,
    pub country_code: [u8; 3],
    pub reserved1: [u8; 21],
}