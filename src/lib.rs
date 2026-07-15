//! A no_std parser for the SBF (Septentrio Binary Format) using the
//! [sans-io](https://sans-io.readthedocs.io/) philosophy.
//!
//! ## `std` BufReader Iterator
//! There is also a `std` API that exposes an `SbfReader` that uses a
//! BufReader. The `SbfReader` implements an `Iterator` that will give
//! you `libsbf::Messages`. To enable this do `cargo add libsbf -F std`

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
use binrw::{binrw, BinRead, BinWrite};

extern crate alloc;

pub mod messages;
pub mod parser;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod reader;

mod binrw_util;

#[cfg(all(test, feature = "std"))]
mod mega_test;

// Re-export all message types at crate level
pub use messages::{
    AGCState, AttCovEuler, AttEuler, AttitudeMode, AuxAntPositionSub, AuxAntPositions,
    BaseVectorCart, BaseVectorGeod, BaselineError, BDSIon, ChannelSatInfo, ChannelStateInfo,
    ChannelStatus, Commands, Comment, DOP, Datum, DiffCorrIn, DiffCorrType, DiskData, DiskStatus,
    EndOfAtt, EndOfMeas, EndOfPVT,
    EventPolarity, EventSource, ExtError, ExtEvent, ExtEventINSNavCart, ExtEventINSNavCartAtt,
    ExtEventINSNavCartAttStdDev, ExtEventINSNavCartPosStdDev, ExtEventINSNavCartVel,
    ExtEventINSNavCartVelStdDev, ExtEventINSNavGeod, ExtEventINSNavGeodAtt,
    ExtEventINSNavGeodAttStdDev, ExtEventINSNavGeodPosStdDev, ExtEventINSNavGeodVel,
    ExtEventINSNavGeodVelStdDev, ExtSensorInfo, ExtSensorMeas, ExtSensorMeasAcceleration,
    ExtSensorMeasAngularRate, ExtSensorMeasInfo, ExtSensorMeasSet, ExtSensorMeasSetType,
    ExtSensorMeasVelocity, ExtSensorMeasZeroVelocityFlag, ExtSensorStatus, GALGstGps, GALIon,
    GALNav, GALUtc, GEONav, GEORawL1, GPSCNav, GPSIon, GPSNav, GPSUtc, GnssMode, INSCouplingMode,
    INSError, INSNavCart, INSNavCartAtt, INSNavCartAttCov, INSNavCartAttStdDev, INSNavCartPosCov,
    INSNavCartPosStdDev, INSNavCartVel, INSNavCartVelCov, INSNavCartVelStdDev, INSNavGeod,
    INSNavGeodAtt, INSNavGeodAttCov, INSNavGeodAttStdDev, INSNavGeodPosCov, INSNavGeodPosStdDev,
    INSNavGeodVel, INSNavGeodVelCov, INSNavGeodVelStdDev, INSSolutionLocation, INSSupport,
    ImuSetup, Meas3Doppler, Meas3Ranges, MeasEpoch, MeasEpochChannelType1, MeasEpochChannelType2,
    MeasExtra, MeasExtraChannelSub, NavCart, PVTCartesian, PVTGeodetic, PosCart, PosCovCartesian,
    PosCovGeodetic, PvtError, PvtMode, PvtModeFlags, QualityInd, QualityIndicator, RFBand,
    RFStatus, RaimIntegrity, ReceiverSetup, ReceiverStatus, ReceiverTime, RiseSet, RxError, RxMessage,
    RxState, SatInfo, SatVisibility, VectorInfoCart, VectorInfoGeod, VelCovCartesian,
    VelCovGeodetic, VelSensorSetup, WACorrFlags, XPPSOffset,
};

// Re-export datagram parser
pub use parser::{parse_datagram, DatagramError, MAX_UDP_PAYLOAD};

#[binrw]
#[derive(Debug)]
struct Id {
    pub bytes: u16,
}

impl Id {
    fn message_type(&self) -> MessageKind {
        MessageKind::from(self.block_number())
    }

    fn block_number(&self) -> u16 {
        // NOTE: Bits 0-12 are the actual Block Number
        self.bytes & 0x1FFF
    }

    fn _block_rev_number(&self) -> u16 {
        // NOTE: Bits 13-15 are the Block Revision Number
        self.bytes & 0xE000
    }
}

#[binrw]
#[derive(Debug)]
struct Header {
    pub crc: u16,
    pub block_id: Id,
    // NOTE: By definition the length includes the sync, crc, and id fields and the
    // actual lenth of a block is `length - 8`.
    pub length: u16,
}

/// Wrapper for a single SBF sub-block to allow padding of individual elements,
/// which is not supported on standalone structs.
#[binrw]
#[derive(Clone, Debug)]
#[brw(import(sb_len: usize))]
pub(crate) struct SubBlock<T>
where
    T: 'static,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    #[brw(pad_size_to = sb_len)]
    data: T,
}

impl<T> From<T> for SubBlock<T>
where
    T: 'static,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    fn from(data: T) -> Self {
        Self { data }
    }
}

impl<T> SubBlock<T>
where
    T: 'static,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    // Neither `From<SubBlock<T>> for T` (orphan rule) nor `Into<T> for
    // SubBlock<T>` (collides with core's blanket `Into`) is allowed, so the
    // unwrap direction is an inherent method.
    pub(crate) fn into_inner(self) -> T {
        self.data
    }
}

/// The count of nested second-level sub-blocks carried by a first-level header,
/// so [`NestedBlock`] can read that many without naming the count field.
pub(crate) trait NestedHeader {
    fn nested_count(&self) -> usize;
}

/// One first-level sub-block of a two-level SBF block: a fixed header padded to
/// SB1Length followed by the nested second-level sub-blocks, each padded to
/// SB2Length.
#[binrw]
#[derive(Clone, Debug)]
#[brw(import(sb1_len: usize, sb2_len: usize))]
pub(crate) struct NestedBlock<H, T>
where
    H: 'static,
    T: 'static,
    for<'a> H: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + NestedHeader + Clone,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + Clone,
{
    #[brw(pad_size_to = sb1_len)]
    pub header: H,
    #[br(args { count: header.nested_count(), inner: (sb2_len,) })]
    #[bw(args_raw = (sb2_len,))]
    pub items: alloc::vec::Vec<SubBlock<T>>,
}

macro_rules! define_messages {
    ($($variant:ident => $code:literal,)+) => {
        /// Typed enum that can be used to determine the type of message
        /// received.
        #[derive(Debug)]
        enum MessageKind {
            $( $variant, )+
            Unsupported,
        }

        impl From<u16> for MessageKind {
            fn from(block_number: u16) -> Self {
                match block_number {
                    $( $code => MessageKind::$variant, )+
                    _ => MessageKind::Unsupported,
                }
            }
        }

        /// Detailed enum that holds the associated payload.
        #[derive(Clone, Debug)]
        pub enum Messages {
            $( $variant($variant), )+
            Unsupported(u16),
        }

        impl Messages {
            pub fn type_name(&self) -> &'static str {
                match self {
                    $( Messages::$variant(_) => stringify!($variant), )+
                    Messages::Unsupported(_) => "Unsupported",
                }
            }

            pub fn tow(&self) -> Option<u32> {
                match self {
                    $( Messages::$variant(m) => m.tow, )+
                    Messages::Unsupported(_) => None,
                }
            }

            pub fn wnc(&self) -> Option<u16> {
                match self {
                    $( Messages::$variant(m) => m.wnc, )+
                    Messages::Unsupported(_) => None,
                }
            }
        }
    };
}

define_messages!(
    MeasExtra => 4000,
    DOP => 4001,
    GALNav => 4002,
    PVTCartesian => 4006,
    PVTGeodetic => 4007,
    SatVisibility => 4012,
    ChannelStatus => 4013,
    ReceiverStatus => 4014,
    Commands => 4015,
    GEORawL1 => 4020,
    MeasEpoch => 4027,
    BaseVectorGeod => 4028,
    GALIon => 4030,
    GALUtc => 4031,
    GALGstGps => 4032,
    GPSCNav => 4042,
    BaseVectorCart => 4043,
    PosCart => 4044,
    ExtSensorMeas => 4050,
    DiskStatus => 4059,
    INSSupport => 4077,
    QualityInd => 4082,
    RFStatus => 4092,
    RxMessage => 4103,
    Meas3Ranges => 4109,
    Meas3Doppler => 4111,
    BDSIon => 4120,
    ExtSensorInfo => 4222,
    ExtSensorStatus => 4223,
    ImuSetup => 4224,
    INSNavCart => 4225,
    INSNavGeod => 4226,
    ExtEventINSNavCart => 4229,
    ExtEventINSNavGeod => 4230,
    VelSensorSetup => 4244,
    NavCart => 4272,
    GPSNav => 5891,
    GPSIon => 5893,
    GPSUtc => 5894,
    GEONav => 5896,
    ReceiverSetup => 5902,
    PosCovCartesian => 5905,
    PosCovGeodetic => 5906,
    VelCovCartesian => 5907,
    VelCovGeodetic => 5908,
    XPPSOffset => 5911,
    ReceiverTime => 5914,
    DiffCorrIn => 5919,
    EndOfPVT => 5921,
    EndOfMeas => 5922,
    ExtEvent => 5924,
    Comment => 5936,
    AttEuler => 5938,
    AttCovEuler => 5939,
    AuxAntPositions => 5942,
    EndOfAtt => 5943,
);

pub fn is_sync(bytes: &[u8; 2]) -> bool {
    bytes == b"$@"
}
