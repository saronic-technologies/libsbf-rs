use alloc::vec::Vec;
use binrw::BinRead;

/// Sub-block for a single auxiliary antenna position.
#[derive(Debug, BinRead)]
pub struct AuxAntPositionSub {
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub nr_sv: Option<u8>,
    pub error: u8,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(x) })]
    pub ambiguity_type: Option<u8>,
    pub aux_ant_id: u8,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub delta_east: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub delta_north: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub delta_up: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub east_vel: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub north_vel: Option<f64>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub up_vel: Option<f64>,
}

// AuxAntPositions Block 5942
#[derive(Debug, BinRead)]
pub struct AuxAntPositions {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    n: u8,
    pub sb_length: u8,
    #[br(count = usize::from(n))]
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
