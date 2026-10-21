use crate::util::binrw::{
    map_f8, map_u1, map_u1_enum_opt, map_u2, map_u4, unmap_f8, unmap_u1, unmap_u1_enum_opt,
    unmap_u2, unmap_u4, unwrap_subblocks, wrap_subblocks,
};
use alloc::vec::Vec;
use binrw::binrw;
use num_enum::{FromPrimitive, IntoPrimitive};

/// Carrier phase ambiguity resolution behind an aux antenna position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum AmbiguityType {
    /// Position obtained with fixed ambiguities.
    Fixed = 0,
    /// Position obtained with float ambiguities.
    Float = 1,
    /// Ambiguity type code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Sub-block for a single auxiliary antenna position.
#[binrw]
#[derive(Clone, Debug)]
pub struct AuxAntPositionSub {
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nr_sv: Option<u8>,
    pub error: u8,
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub ambiguity_type: Option<AmbiguityType>,
    pub aux_ant_id: u8,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_east: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_north: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub delta_up: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub east_vel: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub north_vel: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub up_vel: Option<f64>,
}

// AuxAntPositions Block 5942
#[binrw]
#[derive(Clone, Debug)]
pub struct AuxAntPositions {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = wrap_subblocks)]
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
