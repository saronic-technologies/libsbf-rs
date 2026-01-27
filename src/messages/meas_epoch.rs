use alloc::vec::Vec;
use binrw::binrw;

// MeasEpoch Block 4027
#[binrw]
#[derive(Debug)]
pub struct MeasEpoch {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n1: u8,
    pub sb1_length: u8,
    pub sb2_length: u8,
    pub common_flags: u8,
    pub cum_clk_jumps: u8,
    pub rev1: u8,
    #[br(count = n1)]
    pub channel_type1: Vec<MeasEpochChannelType1>,
}

#[binrw]
#[derive(Debug)]
pub struct MeasEpochChannelType1 {
    pub rx_channel: u8,
    pub type_field: u8,
    pub svid: u8,
    pub misc: u8,
    pub code_lsb: u32,
    #[br(map = |x: i32| if x == -2147483648 { None } else { Some(x) })]
    pub doppler: Option<i32>,
    pub carrier_lsb: u16,
    pub carrier_msb: i8,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub cn0: Option<u8>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub lock_time: Option<u16>,
    pub obs_info: u8,
    pub n2: u8,
    #[br(align_after = 4)]
    #[br(count = n2)]
    pub channel_type2: Vec<MeasEpochChannelType2>,
}

#[binrw]
#[derive(Debug)]
pub struct MeasEpochChannelType2 {
    pub type_field: u8,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub lock_time: Option<u8>,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub cn0: Option<u8>,
    pub offsets_msb: u8,
    pub carrier_msb: i8,
    pub obs_info: u8,
    pub code_offset_lsb: u16,
    pub carrier_lsb: u16,
    pub doppler_offset_lsb: u16,
    #[br(align_after = 4)]
    #[bw(align_after = 4)]
    pub _padding: (),
}
