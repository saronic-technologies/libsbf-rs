use super::pvt_geodetic::{Datum, PvtError, PvtMode, PvtModeFlags};
use crate::util::binrw::{
    map_f8, map_u1_enum_opt, map_u2, map_u4, unmap_f8, unmap_u1_enum_opt, unmap_u2, unmap_u4,
};
use alloc::vec::Vec;
use binrw::binrw;

// PosLocal Block 4052
#[binrw]
#[derive(Clone, Debug)]
pub struct PosLocal {
    /// Receiver time stamp, in 0.001 s.
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    /// Receiver time stamp, in weeks.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Bit field indicating the GNSS PVT mode.
    mode_raw: u8,
    /// PVT error code.
    #[br(map = |x: u8| PvtError::from(x))]
    #[bw(map = |x: &PvtError| u8::from(*x))]
    pub error: PvtError,
    /// Latitude in radians, from -π/2 to +π/2, positive North of Equator.
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub latitude: Option<f64>,
    /// Longitude in radians, from -π to +π, positive East of Greenwich.
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub longitude: Option<f64>,
    /// Height in meters. See the HeightType field of the corresponding RTCMDatum block for
    /// the interpretation of the height.
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub height: Option<f64>,
    /// Reference frame to which the position relates. If the value is in the 20 to 24 range,
    /// the corresponding datum parameters can be found in the RTCMDatum block having a
    /// matching Datum field. Value 25 corresponds to the local coordinate reference system
    /// selected with the setLocalCoordOperation command.
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub datum: Option<Datum>,
    /// Padding bytes.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl PosLocal {
    /// PVT mode (bits 0-3 of mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.mode_raw)
    }

    /// Mode flags (bits 6-7 of mode).
    pub fn mode_flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.mode_raw)
    }
}
