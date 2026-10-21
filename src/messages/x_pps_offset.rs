use crate::binrw_util;
use binrw::binrw;
use num_enum::{FromPrimitive, IntoPrimitive};

/// Time scale an xPPS pulse is referenced to, as set with the setPPSParameters command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
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
#[derive(Clone, Debug)]
pub struct XPPSOffset {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    /// Age in seconds of the last synchronization to the time scale.
    pub sync_age: u8,
    #[br(map = |x: u8| PpsTimeScale::from(x))]
    #[bw(map = |x: &PpsTimeScale| u8::from(*x))]
    pub time_scale: PpsTimeScale,
    /// Offset of the xPPS pulse from the reference in nanoseconds. Negative when the pulse is in advance.
    pub offset: f32,
}
