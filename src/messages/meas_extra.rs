use crate::SubBlock;
use alloc::vec::Vec;
use binrw::binrw;

// MeasExtra Block 4000
#[binrw]
#[derive(Clone, Debug)]
pub struct MeasExtra {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(crate::DO_NOT_USE_U4))]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    pub doppler_var_factor: f32,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) },
         map = |v: Vec<SubBlock<MeasExtraChannelSub>>| v.into_iter().map(SubBlock::into_inner).collect())]
    #[bw(args_raw = (usize::from(*sb_length),),
         map = |v: &Vec<MeasExtraChannelSub>| v.iter().cloned().map(SubBlock::from).collect::<Vec<_>>())]
    pub channel_sub: Vec<MeasExtraChannelSub>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct MeasExtraChannelSub {
    pub rx_channel: u8,
    pub type_field: u8,
    pub mp_correction: i16,
    pub smoothing_corr: i16,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub code_var: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub carrier_var: Option<u16>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub lock_time: Option<u16>,
    pub cum_loss_cont: u8,
    pub car_mp_corr: i8,
    pub info: u8,
    pub misc: u8,
}
