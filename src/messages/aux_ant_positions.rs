use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

/// Sub-block for a single auxiliary antenna position.
#[binrw]
#[derive(Clone, Debug)]
pub struct AuxAntPositionSub {
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub nr_sv: Option<u8>,
    pub error: u8,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub ambiguity_type: Option<u8>,
    pub aux_ant_id: u8,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub delta_east: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub delta_north: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub delta_up: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub east_vel: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub north_vel: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub up_vel: Option<f64>,
}

// AuxAntPositions Block 5942
#[binrw]
#[derive(Clone, Debug)]
pub struct AuxAntPositions {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    n: u8,
    pub sb_length: u8,
    #[br(args { count: usize::from(n), inner: (usize::from(sb_length),) }, map = binrw_util::unwrap_subblocks)]
    #[bw(args_raw = (usize::from(*sb_length),), map = binrw_util::wrap_subblocks)]
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
