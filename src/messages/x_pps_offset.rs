use crate::util::binrw::{map_u1_enum, map_u2, map_u4, unmap_u1_enum, unmap_u2, unmap_u4};
use binrw::binrw;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::Serialize;

/// Time scale an xPPS pulse is referenced to, as set with the setPPSParameters command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Serialize)]
#[repr(u8)]
pub enum PpsTimeScale {
    /// GPS time.
    Gps = 1,
    /// UTC.
    Utc = 2,
    /// Receiver time.
    Receiver = 3,
    /// GLONASS time.
    Glonass = 4,
    /// Galileo time.
    Galileo = 5,
    /// BeiDou time.
    BeiDou = 6,
    /// Fugro AtomiChron time.
    Fugro = 100,
    /// Time scale code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

// xPPSOffset Block 5911
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct XPPSOffset {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Age in seconds of the last synchronization to the time scale.
    pub sync_age: u8,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub time_scale: PpsTimeScale,
    /// Offset of the xPPS pulse from the reference in nanoseconds. Negative when the pulse is in advance.
    pub offset: f32,
}
