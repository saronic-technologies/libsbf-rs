use super::common::RawAttitudeError;
use crate::util::binrw::{map_f4, map_u2, map_u4, unmap_f4, unmap_u2, unmap_u4};
use binrw::binrw;

// AttCovEuler Block 5939
#[binrw]
#[derive(Clone, Debug)]
pub struct AttCovEuler {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub reserved: u8,
    /// Bit field providing error information.
    pub error: RawAttitudeError,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_head_head: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_pitch_pitch: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_roll_roll: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_head_pitch: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_head_roll: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cov_pitch_roll: Option<f32>,
}
