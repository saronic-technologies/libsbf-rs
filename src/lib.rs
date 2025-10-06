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

pub mod parser;
pub mod messages;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod reader;

// Constants for DO_NOT_USE values
const DO_NOT_USE_I2: i16 = -32768;
const DO_NOT_USE_U1: u8  = 255;
const DO_NOT_USE_U2: u16 = 65535;
const DO_NOT_USE_U4: u32 = 4294967295;
const DO_NOT_USE_F4: f32 = -2e10;
const DO_NOT_USE_F8: f64 = -2e10;

// Re-export all message types at crate level
pub use messages::{
    MeasEpoch, MeasEpochChannelType1, MeasEpochChannelType2,
    MeasExtra, MeasExtraChannelSub,
    Meas3Ranges, Meas3Doppler,
    DiffCorrIn,
    INSSupport,
    INSNavGeod, INSNavGeodPosStdDev, INSNavGeodAtt, INSNavGeodAttStdDev,
    INSNavGeodVel, INSNavGeodVelStdDev, INSNavGeodPosCov, INSNavGeodVelCov,
    INSNavGeodAttCov,
    AttEuler, AttCovEuler,
    ExtSensorMeas, ExtSensorMeasSet, ExtSensorMeasAcceleration,
    ExtSensorMeasAngularRate, ExtSensorMeasVelocity, ExtSensorMeasInfo,
    ExtSensorMeasZeroVelocityFlag, ExtSensorMeasSetType,
    QualityInd,
    ImuSetup,
    ReceiverSetup,
    GEORawL1, GEONav,
    PosCovGeodetic,
    PVTGeodetic,
    ReceiverStatus, AGCState,
    ExtSensorStatus,
    GALIon,
    GALUtc,
    GPSIon,
    GPSUtc,
    VelSensorSetup,
    ExtSensorInfo,
    GALNav,
    GALGstGps,
    GPSNav,
    Commands,
};

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
        #[derive(Debug)]
        pub enum Messages {
            $( $variant($variant), )+
            Unsupported(u16),
        }
    };
}

define_messages!(
    MeasExtra => 4000,
    GALNav => 4002,
    PVTGeodetic => 4007,
    ReceiverStatus => 4014,
    Commands => 4015,
    GEORawL1 => 4020,
    MeasEpoch => 4027,
    GALIon => 4030,
    GALUtc => 4031,
    GALGstGps => 4032,
    INSSupport => 4077,
    Meas3Ranges => 4109,
    Meas3Doppler => 4111,
    ExtSensorStatus => 4223,
    INSNavGeod => 4226,
    VelSensorSetup => 4244,
    AttEuler => 5938,
    AttCovEuler => 5939,
    DiffCorrIn => 5919,
    ExtSensorMeas => 4050,
    QualityInd => 4082,
    ExtSensorInfo => 4222,
    ImuSetup => 4224,
    ReceiverSetup => 5902,
    GEONav => 5896,
    GPSIon => 5893,
    GPSNav => 5891,
    GPSUtc => 5894,
    PosCovGeodetic => 5906,
);

pub fn is_sync(bytes: &[u8; 2]) -> bool {
    bytes == b"$@"
}