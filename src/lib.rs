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
use alloc::vec::Vec;

pub mod parser;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod reader;

const DO_NOT_USE_I2: i16 = -32768;
const DO_NOT_USE_U1: u8  = 255;
const DO_NOT_USE_U2: u16 = 65535;
const DO_NOT_USE_U4: u32 = 4294967295;
const DO_NOT_USE_F4: f32 = -2e10;
const DO_NOT_USE_F8: f64 = -2e10;



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
    pub length: u16,
}

macro_rules! define_messages {
    ( $( $variant:ident => $code:expr ),+ $(,)? ) => {
        /// Core enum that just represents the message kind.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
            Unsupported,
        }
    };
}

define_messages!(
    INSNavGeod => 4226,
    AttEuler => 5938,
    ExtSensorMeas => 4050,
    QualityInd => 4082,
    ImuSetup => 4224,
    ReceiverSetup => 5902,
    EndOfPvt => 4109,
    Dop => 4000,
    EndOfAux => 4111,
    BaseVectorGeod => 5919,
    QSFQualityInd => 5906,
    GalAuthStatus => 4020,
    VelCovGeodetic => 5891,
);

// Attitude Euler Block 5938
#[binrw]
#[derive(Debug)]
pub struct AttEuler {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    #[br(map = |x| if x == DO_NOT_USE_U1 { None } else { Some(x) })]
    pub nrsv: Option<u8>,
    // TODO: create Error enum
    pub error: u8,
    pub mode: u16,
    _reserved: u16,

    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll: Option<f32>,

    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch_dot: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll_dot: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_dot: Option<f32>,
}

// External Sensor Measurement Block 4050
#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeas {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(count = n)]
    pub ext_sensor_meas_set: Vec<ExtSensorMeasSet>,
}

#[repr(u8)]
pub enum ExtSensorMeasSetType {
    Acceleration = 0,
    AngularRate = 1,
    _Reserved = 2,
    Info = 3,
    Velocity = 4,
    ZeroVelocityFlag = 20,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasSet {
    pub source: u8,
    pub sensor_model: u8,
    pub type_: u8,
    pub obs_info: u8,

    #[br(if(type_ == ExtSensorMeasSetType::Acceleration as u8))]
    pub acc: Option<ExtSensorMeasAcceleration>,
    #[br(if(type_ == ExtSensorMeasSetType::AngularRate as u8))]
    pub ang_rate: Option<ExtSensorMeasAngularRate>,
    #[br(if(type_ == ExtSensorMeasSetType::Info as u8))]
    pub info: Option<ExtSensorMeasInfo>,
    #[br(if(type_ == ExtSensorMeasSetType::Velocity as u8))]
    pub vel: Option<ExtSensorMeasVelocity>,
    #[br(if(type_ == ExtSensorMeasSetType::ZeroVelocityFlag as u8))]
    pub zero_vel_flag: Option<ExtSensorMeasZeroVelocityFlag>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasAcceleration {
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub ax: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub ay: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub az: Option<f64>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasAngularRate {
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub wx: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub wy: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub wz: Option<f64>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasVelocity {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vx: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vy: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vz: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub stdx: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub stdy: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub stdz: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasInfo {
    #[br(map = |x| if x == DO_NOT_USE_I2 { None } else { Some(x) })]
    pub sensor_temp: Option<i16>,
    pub _reserved: [u8; 22],
}

#[binrw]
#[derive(Debug)]
pub struct ExtSensorMeasZeroVelocityFlag {
    pub zero_v_flag: f64,
    pub _reserved: [u8; 16],
}

// INS Nav Geod Block 4226
#[binrw]
#[derive(Debug)]
pub struct INSNavGeod {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    // TODO: create GNSSMode type for future telemetry info
    pub gnss_mode: u8,
    // TODO: create Error enum
    pub error: u8,
    // TODO: unpack this if we want more info
    pub info: u16,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub gnss_age: Option<u16>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub latitude: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub longitude: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub height: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub undulation: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub accuracy: Option<u16>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub latency: Option<u16>,
    // TODO: create a Datum enum
    #[br(map = |x| if x == DO_NOT_USE_U1 { None } else { Some(x) })]
    pub datum: Option<u8>,
    _reserved: u8,
    // TODO: unpack into an SBList type so we know what INSNav Sub Blocks we can parse
    pub sb_list: u16,

    #[br(if((sb_list >> 0) & 1 == 1))]
    pub pos_std_dev: Option<INSNavGeodPosStdDev>,
    #[br(if((sb_list >> 1) & 1 == 1))]
    pub att: Option<INSNavGeodAtt>,
    #[br(if((sb_list >> 2) & 1 == 1))]
    pub att_std_dev: Option<INSNavGeodAttStdDev>,
    #[br(if((sb_list >> 3) & 1 == 1))]
    pub vel: Option<INSNavGeodVel>,
    #[br(if((sb_list >> 4) & 1 == 1))]
    pub vel_std_dev: Option<INSNavGeodVelStdDev>,
    #[br(if((sb_list >> 5) & 1 == 1))]
    pub pos_cov: Option<INSNavGeodPosCov>,
    #[br(if((sb_list >> 6) & 1 == 1))]
    pub att_cov: Option<INSNavGeodAttCov>,
    #[br(if((sb_list >> 7) & 1 == 1))]
    pub vel_cov: Option<INSNavGeodVelCov>,
}

#[binrw]
#[derive(Debug, PartialEq)]
pub struct QualityInd {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub reserved: u8,
    #[br( if(n > 0))]
    pub indicator_1: Option<u16>,
    #[br(if(n > 1))]
    pub indicator_2: Option<u16>,
    #[br(if(n > 2))]
    pub indicator_3: Option<u16>,
    #[br(if(n > 3))]
    pub indicator_4: Option<u16>,
    #[br(if(n > 4))]
    pub indicator_5: Option<u16>,
    #[br(if(n > 5))]
    pub indicator_6: Option<u16>,
    #[br(if(n > 6))]
    pub indicator_7: Option<u16>,
    #[br( if(n > 7))]
    pub indicator_8: Option<u16>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodPosStdDev {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub longitude_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub latitude_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub height_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAtt {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAttStdDev {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVel {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vu: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVelStdDev {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vu_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodPosCov {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub latitude_longitude_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub latitude_height_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub longitude_height_cov: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVelCov {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve_vn_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve_vu_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn_vu_cov: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAttCov {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_pitch_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_roll_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch_roll_cov: Option<f32>,
}

// IMU Setup Block 4224
#[binrw]
#[derive(Debug)]
pub struct ImuSetup {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    _reserved: u8,
    // TODO: create SerialPort enum for future serial port info
    pub serial_port: u8,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ant_lever_arm_x_m: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ant_lever_arm_y_m: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ant_lever_arm_z_m: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub theta_x_deg: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub theta_y_deg: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub theta_z_deg: Option<f32>,
}

// Receiver Setup Block 5902
#[binrw]
#[derive(Debug)]
pub struct ReceiverSetup {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub reserved: [u8; 2],
    pub marker_name: [u8; 60],
    pub marker_number: [u8; 20],
    pub observer: [u8; 20],
    pub agency: [u8; 40],
    pub rx_serial_number: [u8; 20],
    pub rx_name: [u8; 20],
    pub rx_version: [u8; 20],
    pub ant_serial_nbr: [u8; 20],
    pub ant_type: [u8; 20],
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_h: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_e: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub delta_n: Option<f32>,
    pub marker_type: [u8; 20],
    pub gnss_fw_version: [u8; 40],
    pub product_name: [u8; 40],
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub latitude: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x) })]
    pub longitude: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x) })]
    pub height: Option<f32>,
    pub station_code: [u8; 10],
    pub monument_idx: u8,
    pub receiver_idx: u8,
    pub country_code: [u8; 3],
    pub reserved1: [u8; 21],
}

// EndOfPVT Block 4109
#[binrw]
#[derive(Debug)]
pub struct EndOfPvt {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
}

// DOP Block 4000
#[binrw]
#[derive(Debug)]
pub struct Dop {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub nr_sv: u8,
    pub reserved: u8,
    pub pdop: u16,
    pub tdop: u16,
    pub hdop: u16,
    pub vdop: u16,
    pub hpl: f32,
    pub vpl: f32,
}

// EndOfAux Block 4111
#[binrw]
#[derive(Debug)]
pub struct EndOfAux {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
}

// BaseVectorGeod Block 5919
#[binrw]
#[derive(Debug)]
pub struct BaseVectorGeod {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_length: u8,
    #[br(count = n)]
    pub base_vectors: Vec<BaseVectorInfo>,
}

#[binrw]
#[derive(Debug)]
pub struct BaseVectorInfo {
    pub nr_sv: u8,
    pub error: u8,
    pub mode: u8,
    pub misc: u8,
    pub delta_east: f64,
    pub delta_north: f64,
    pub delta_up: f64,
    pub delta_veast: f32,
    pub delta_vnorth: f32,
    pub delta_vup: f32,
    pub azimuth: u16,
    pub elevation: i16,
    pub ref_id: u16,
    pub corr_age: u16,
    pub signal_info: u32,
}

// QSFQualityInd Block 5906 - Quality Sub-Frame Quality Indicators
#[binrw]
#[derive(Debug)]
pub struct QSFQualityInd {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub n: u8,
    pub sb_size: u8,
    pub _reserved: [u8; 2],
    #[br(count = n)]
    pub indicators: Vec<u16>,
}

// GALAuthStatus Block 4020
#[binrw]
#[derive(Debug)]
pub struct GalAuthStatus {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub osnma_status: u8,
    pub trusted_time_delta: f32,
    pub gal_active_mask: u64,
    pub gal_authentic_mask: u64,
    pub gps_active_mask: u64,
    pub gps_authentic_mask: u64,
}

// VelCovGeodetic Block 5891
#[binrw]
#[derive(Debug)]
pub struct VelCovGeodetic {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub mode: u8,
    pub error: u8,
    pub cov_veve: f32,
    pub cov_vnvn: f32,
    pub cov_vuvu: f32,
    pub cov_dtdt: f32,
    pub cov_vevn: f32,
    pub cov_vevu: f32,
    pub cov_vedt: f32,
    pub cov_vnvu: f32,
    pub cov_vndt: f32,
    pub cov_vudt: f32,
}

pub fn is_sync(bytes: &[u8; 2]) -> bool {
    bytes == b"$@"
}
