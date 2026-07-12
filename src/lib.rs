//! A no_std parser for the SBF (Septentrio Binary Format) using the
//! [sans-io](https://sans-io.readthedocs.io/) philosophy.
//!
//! ## `std` BufReader Iterator
//! There is also a `std` API that exposes an `SbfReader` that uses a
//! BufReader. The `SbfReader` implements an `Iterator` that will give
//! you `libsbf::Messages`. To enable this do `cargo add libsbf -F std`

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
use binrw::binrw;

extern crate alloc;

pub mod messages;
pub mod parser;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod reader;

#[cfg(all(test, feature = "std"))]
mod mega_test;

// Constants for DO_NOT_USE values
const DO_NOT_USE_I1: i8 = -128;
const DO_NOT_USE_I2: i16 = -32768;
const DO_NOT_USE_U1: u8 = 255;
const DO_NOT_USE_U2: u16 = 65535;
const DO_NOT_USE_U4: u32 = 4294967295;
const DO_NOT_USE_F4: f32 = -2e10;
const DO_NOT_USE_F8: f64 = -2e10;

// Re-export all message types at crate level
pub use messages::{
    AGCState, AttCovEuler, AttEuler, AttitudeMode, AuxAntPositionSub, AuxAntPositions,
    BaseVectorCart, BaselineError, BDSIon, Commands, DOP, Datum, DiffCorrIn, DiffCorrType, EndOfMeas,
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
    RFStatus, RaimIntegrity, ReceiverSetup, ReceiverStatus, RxError, RxState, VectorInfoCart,
    VelCovCartesian, VelCovGeodetic, VelSensorSetup, WACorrFlags,
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
    ReceiverStatus => 4014,
    Commands => 4015,
    GEORawL1 => 4020,
    MeasEpoch => 4027,
    GALIon => 4030,
    GALUtc => 4031,
    GALGstGps => 4032,
    BaseVectorCart => 4043,
    PosCart => 4044,
    GPSCNav => 4042,
    INSSupport => 4077,
    Meas3Ranges => 4109,
    Meas3Doppler => 4111,
    NavCart => 4272,
    BDSIon => 4120,
    ExtSensorStatus => 4223,
    INSNavCart => 4225,
    INSNavGeod => 4226,
    ExtEventINSNavCart => 4229,
    ExtEventINSNavGeod => 4230,
    VelSensorSetup => 4244,
    AttEuler => 5938,
    AttCovEuler => 5939,
    AuxAntPositions => 5942,
    DiffCorrIn => 5919,
    ExtSensorMeas => 4050,
    RFStatus => 4092,
    QualityInd => 4082,
    ExtSensorInfo => 4222,
    ImuSetup => 4224,
    ReceiverSetup => 5902,
    GEONav => 5896,
    GPSIon => 5893,
    GPSNav => 5891,
    GPSUtc => 5894,
    PosCovCartesian => 5905,
    PosCovGeodetic => 5906,
    VelCovCartesian => 5907,
    VelCovGeodetic => 5908,
    EndOfMeas => 5922,
    ExtEvent => 5924,
);

pub fn is_sync(bytes: &[u8; 2]) -> bool {
    bytes == b"$@"
}
