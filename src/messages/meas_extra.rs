use binrw::binrw;
use alloc::vec::Vec;

// MeasExtra Block 4000
#[binrw]
#[derive(Debug, Clone)]
pub struct MeasExtra {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    pub doppler_var_factor: f32,
    #[br(count = n)]
    pub channel_sub: Vec<MeasExtraChannelSub>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct MeasExtraChannelSub {
    pub rx_channel: u8,
    pub type_field: u8,
    pub mp_correction: i16,
    pub smoothing_corr: i16,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub code_var: Option<u16>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub carrier_var: Option<u16>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub lock_time: Option<u16>,
    pub cum_loss_cont: u8,
    pub car_mp_corr: i8,
    pub info: u8,
    pub misc: u8,
    #[br(align_after = 4)]
    #[bw(align_after = 4)]
    pub _padding: (),
}