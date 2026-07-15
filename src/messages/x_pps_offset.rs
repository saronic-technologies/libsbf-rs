use binrw::binrw;

// xPPSOffset Block 5911
#[binrw]
#[derive(Clone, Debug)]
pub struct XPPSOffset {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    /// Age in seconds of the last synchronization to the time scale.
    pub sync_age: u8,
    /// Time scale the xPPS is referenced to: 1 GNSS, 2 UTC, 3 receiver, 4 GLONASS.
    pub time_scale: u8,
    /// Offset of the xPPS pulse from the reference in nanoseconds. Negative when the pulse is in advance.
    pub offset: f32,
}
