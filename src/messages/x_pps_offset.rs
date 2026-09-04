use crate::binrw_util;
use binrw::binrw;
use serde::Serialize;

// xPPSOffset Block 5911
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct XPPSOffset {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    /// Age in seconds of the last synchronization to the time scale.
    pub sync_age: u8,
    /// Time scale the xPPS is referenced to: 1 GNSS, 2 UTC, 3 receiver, 4 GLONASS.
    pub time_scale: u8,
    /// Offset of the xPPS pulse from the reference in nanoseconds. Negative when the pulse is in advance.
    pub offset: f32,
}
