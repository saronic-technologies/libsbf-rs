use crate::SubBlock;
use alloc::vec::Vec;
use binrw::binrw;

/// Sub-block for a single auxiliary antenna position.
#[binrw]
#[derive(Clone, Debug)]
pub struct AuxAntPositionSub {
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u8>| x.unwrap_or(crate::DO_NOT_USE_U1))]
    pub nr_sv: Option<u8>,
    pub error: u8,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u8>| x.unwrap_or(crate::DO_NOT_USE_U1))]
    pub ambiguity_type: Option<u8>,
    pub aux_ant_id: u8,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub delta_east: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub delta_north: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub delta_up: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub east_vel: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub north_vel: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    #[bw(map = |x: &Option<f64>| x.unwrap_or(crate::DO_NOT_USE_F8))]
    pub up_vel: Option<f64>,
}

// AuxAntPositions Block 5942
#[binrw]
#[derive(Clone, Debug)]
pub struct AuxAntPositions {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u32>| x.unwrap_or(crate::DO_NOT_USE_U4))]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    #[bw(map = |x: &Option<u16>| x.unwrap_or(crate::DO_NOT_USE_U2))]
    pub wnc: Option<u16>,
    n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) },
         map = |v: Vec<SubBlock<AuxAntPositionSub>>| v.into_iter().map(SubBlock::into_inner).collect())]
    #[bw(args_raw = (usize::from(*sb_length),),
         map = |v: &Vec<AuxAntPositionSub>| v.iter().cloned().map(SubBlock::from).collect::<Vec<_>>())]
    pub aux_ant_positions: Vec<AuxAntPositionSub>,
    #[br(parse_with = binrw::helpers::until_eof)]
    _padding: Vec<u8>,
}

impl AuxAntPositions {
    /// Number of auxiliary antenna sub-blocks.
    pub fn num_antennas(&self) -> u8 {
        self.n
    }
}
