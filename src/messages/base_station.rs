use super::common::Datum;
use crate::util::binrw::{
    map_u1_enum, map_u1_enum_opt, map_u2, map_u4, unmap_u1_enum, unmap_u1_enum_opt, unmap_u2,
    unmap_u4,
};
use alloc::vec::Vec;
use binrw::binrw;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::Serialize;

/// Base station type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Serialize)]
#[repr(u8)]
pub enum BaseType {
    /// Fixed base station.
    Fixed = 0,
    /// Moving base station, reserved for future use.
    Moving = 1,
    /// Base type code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Source of the base station coordinates. It decides how to interpret the X, Y, Z
/// coordinates: the L1 phase center for RTCM 2.x message 3, CMR 2.x message 1 and CMR+
/// type 2, the antenna reference point for RTCM 2.x message 24 and RTCM 3.x messages 1005
/// and 1006, and a proprietary point for RTCMV message 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Serialize)]
#[repr(u8)]
pub enum BaseStationSource {
    /// RTCM 2.x, message 3.
    Rtcm2Msg3 = 0,
    /// RTCM 2.x, message 24.
    Rtcm2Msg24 = 2,
    /// CMR 2.x, message 1.
    Cmr2Msg1 = 4,
    /// RTCM 3.x, message 1005 or 1006.
    Rtcm3Msg1005Or1006 = 8,
    /// RTCMV, message 3.
    RtcmvMsg3 = 9,
    /// CMR+, type 2.
    CmrPlusType2 = 10,
    /// Source code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

// BaseStation Block 5949
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct BaseStation {
    /// Receiver time stamp, in 0.001 s.
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    /// Receiver time stamp, in weeks.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// The base station ID.
    pub base_station_id: u16,
    /// Base station type, None when the receiver reports it as unknown.
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub base_type: Option<BaseType>,
    /// Source of the base station coordinates.
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub source: BaseStationSource,
    /// Datum in which the coordinates are expressed. Firmware prior to 1.5.x marks this field
    /// not applicable.
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub datum: Option<Datum>,
    /// Reserved for future use, to be ignored by decoding software.
    _reserved: u8,
    /// Antenna X coordinate in meters, expressed in the datum specified by the Datum field.
    pub x: f64,
    /// Antenna Y coordinate in meters.
    pub y: f64,
    /// Antenna Z coordinate in meters.
    pub z: f64,
    /// Padding bytes.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::{BaseStation, BaseStationSource};
    use crate::Datum;
    use alloc::vec::Vec;
    use binrw::{io::Cursor, BinRead, BinWrite};

    // A body recorded from a receiver taking RTCM 3.x base station coordinates.
    #[test]
    fn reads_recorded_block() {
        let body: [u8; 36] = [
            0x84, 0x14, 0xfb, 0x07, 0x72, 0x09, 0x00, 0x00, 0xff, 0x08, 0x13, 0x00, 0xc8, 0x98,
            0xbb, 0x96, 0x87, 0x59, 0xf5, 0x40, 0x99, 0x4c, 0x15, 0xb0, 0x73, 0x02, 0x55, 0xc1,
            0x48, 0xe1, 0x7a, 0x8c, 0x4f, 0x73, 0x48, 0x41,
        ];

        let block = BaseStation::read_le(&mut Cursor::new(&body)).unwrap();
        assert_eq!(block.tow, Some(133895300));
        assert_eq!(block.wnc, Some(2418));
        assert_eq!(block.base_station_id, 0);
        assert_eq!(block.base_type, None);
        assert_eq!(block.source, BaseStationSource::Rtcm3Msg1005Or1006);
        assert_eq!(block.datum, Some(Datum::DgnssBaseStation));
        assert_eq!(block.x, 87448.4743);
        assert_eq!(block.z, 3204767.0975);

        let mut out = Vec::new();
        block.write_le(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(out, body);
    }
}
