use super::pvt_geodetic::Datum;
use crate::util::binrw::{
    map_u1_enum, map_u1_enum_opt, map_u2, map_u4, unmap_u1_enum, unmap_u1_enum_opt, unmap_u2,
    unmap_u4,
};
use alloc::vec::Vec;
use binrw::binrw;
use num_enum::{FromPrimitive, IntoPrimitive};

/// Height Indicator field from RTCM 3.x messages 1021 and 1022, telling how to interpret
/// the height reported in the PosLocal and PosProjected blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum HeightType {
    /// Geometrical height.
    Geometrical = 0,
    /// Physical height, with the height definition of the target CRS.
    PhysicalTarget = 1,
    /// Physical height, with the height definition of the source CRS.
    PhysicalSource = 2,
    /// Height type code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// Maximum approximation error after applying a datum transformation, as one of the
/// quality indicator codes of RTCM 3.x messages 1021 to 1024.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum RtcmDatumQuality {
    /// Unknown quality.
    UnknownQuality = 0,
    /// Quality better than 21 mm, from MT1021/1022.
    Mt1021Below21Mm = 1,
    /// Quality 21 to 50 mm, from MT1021/1022.
    Mt1021From21To50Mm = 2,
    /// Quality 51 to 200 mm, from MT1021/1022.
    Mt1021From51To200Mm = 3,
    /// Quality 201 to 500 mm, from MT1021/1022.
    Mt1021From201To500Mm = 4,
    /// Quality 501 to 2000 mm, from MT1021/1022.
    Mt1021From501To2000Mm = 5,
    /// Quality 2001 to 5000 mm, from MT1021/1022.
    Mt1021From2001To5000Mm = 6,
    /// Quality worse than 5001 mm, from MT1021/1022.
    Mt1021Above5001Mm = 7,
    /// Quality 0 to 10 mm, from MT1023/1024.
    Mt1023From0To10Mm = 9,
    /// Quality 11 to 20 mm, from MT1023/1024.
    Mt1023From11To20Mm = 10,
    /// Quality 21 to 50 mm, from MT1023/1024.
    Mt1023From21To50Mm = 11,
    /// Quality 51 to 100 mm, from MT1023/1024.
    Mt1023From51To100Mm = 12,
    /// Quality 101 to 200 mm, from MT1023/1024.
    Mt1023From101To200Mm = 13,
    /// Quality 201 to 500 mm, from MT1023/1024.
    Mt1023From201To500Mm = 14,
    /// Quality worse than 501 mm, from MT1023/1024.
    Mt1023Above501Mm = 15,
    /// Quality code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl RtcmDatumQuality {
    /// Lower bound of the approximation error in millimeters, None when the quality is
    /// unknown.
    pub fn range_min(&self) -> Option<u16> {
        match self {
            Self::Mt1021Below21Mm | Self::Mt1023From0To10Mm => Some(0),
            Self::Mt1023From11To20Mm => Some(11),
            Self::Mt1021From21To50Mm | Self::Mt1023From21To50Mm => Some(21),
            Self::Mt1021From51To200Mm | Self::Mt1023From51To100Mm => Some(51),
            Self::Mt1023From101To200Mm => Some(101),
            Self::Mt1021From201To500Mm | Self::Mt1023From201To500Mm => Some(201),
            Self::Mt1021From501To2000Mm | Self::Mt1023Above501Mm => Some(501),
            Self::Mt1021From2001To5000Mm => Some(2001),
            Self::Mt1021Above5001Mm => Some(5001),
            Self::UnknownQuality | Self::Unknown(_) => None,
        }
    }

    /// Upper bound of the approximation error in millimeters, None when the quality is
    /// unknown or has no upper bound.
    pub fn range_max(&self) -> Option<u16> {
        match self {
            Self::Mt1023From0To10Mm => Some(10),
            Self::Mt1021Below21Mm | Self::Mt1023From11To20Mm => Some(20),
            Self::Mt1021From21To50Mm | Self::Mt1023From21To50Mm => Some(50),
            Self::Mt1023From51To100Mm => Some(100),
            Self::Mt1021From51To200Mm | Self::Mt1023From101To200Mm => Some(200),
            Self::Mt1021From201To500Mm | Self::Mt1023From201To500Mm => Some(500),
            Self::Mt1021From501To2000Mm => Some(2000),
            Self::Mt1021From2001To5000Mm => Some(5000),
            Self::Mt1021Above5001Mm | Self::Mt1023Above501Mm => None,
            Self::UnknownQuality | Self::Unknown(_) => None,
        }
    }
}

// RTCMDatum Block 4049
#[binrw]
#[derive(Clone, Debug)]
pub struct RTCMDatum {
    /// Receiver time stamp, in 0.001 s.
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    /// Receiver time stamp, in weeks.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Name of the source Coordinate Reference System, right-padded with zeros.
    pub source_crs: [u8; 32],
    /// Name of the target Coordinate Reference System, right-padded with zeros.
    pub target_crs: [u8; 32],
    /// Datum code that the PosLocal and PosProjected blocks use for this transformation.
    /// None when this SourceCRS/TargetCRS pair is currently not used by the receiver.
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub datum: Option<Datum>,
    /// Height Indicator field from MT1021 and MT1022.
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub height_type: HeightType,
    /// Bit field indicating the maximum approximation error after applying the
    /// transformation.
    quality_ind_raw: u8,
    /// Padding bytes.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl RTCMDatum {
    /// Horizontal quality indicator, from bits 0-3 of the quality field.
    pub fn horizontal_quality(&self) -> RtcmDatumQuality {
        RtcmDatumQuality::from(self.quality_ind_raw & 0x0F)
    }

    /// Vertical quality indicator, from bits 4-7 of the quality field.
    pub fn vertical_quality(&self) -> RtcmDatumQuality {
        RtcmDatumQuality::from(self.quality_ind_raw >> 4)
    }
}
