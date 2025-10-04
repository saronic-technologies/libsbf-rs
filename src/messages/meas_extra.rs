use binrw::binrw;
use alloc::vec::Vec;

// MeasExtra Block 4000
#[binrw]
#[derive(Debug)]
pub struct MeasExtra {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    pub doppler_var_factor: f32,
    #[br(count = n)]
    pub channel_sub: Vec<MeasExtraChannelSub>,
}

#[binrw]
#[derive(Debug)]
pub struct MeasExtraChannelSub {
    pub rx_channel: u8,
    pub type_field: u8,
    pub mp_correction: i16,
    pub smoothing_corr: i16,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub code_var: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub carrier_var: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub lock_time: Option<u16>,
    pub cum_loss_cont: u8,
    pub car_mp_corr: i8,
    pub info: u8,
    pub misc: u8,
    #[br(align_after = 4)]
    #[bw(align_after = 4)]
    pub _padding: (),
}