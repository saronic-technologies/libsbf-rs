#![no_std]
use binrw::binrw;

const DO_NOT_USE_U1: u8  = 255;
const DO_NOT_USE_U2: u16 = 65535;
const DO_NOT_USE_U4: u32 = 4294967295;
const DO_NOT_USE_F4: f32 = -2e10;
const DO_NOT_USE_F8: f64 = -2e10;

#[binrw]
#[derive(Debug)]
pub struct Id {
    pub bytes: u16,
}

impl Id {
    pub fn message_type(&self) -> Messages {
        Messages::from(self.block_number())
    }

    pub fn block_number(&self) -> u16 {
        // NOTE: Bits 0-12 are the actual Block Number
        self.bytes & 0x1FFF
    }

    pub fn block_rev_number(&self) -> u16 {
        // NOTE: Bits 13-15 are the Block Revision Number
        self.bytes & 0xE000
    }
}

#[binrw]
#[derive(Debug)]
pub struct Header {
    pub crc: u16,
    pub block_id: Id,
    pub length: u16,
}

pub enum Messages {
    INSNavGeod,
    AttEuler,
    Unsupported,
}

impl From<u16> for Messages {
    fn from(block_number: u16) -> Self {
        match block_number {
            4226 => Self::INSNavGeod,
            5938 => Self::AttEuler,
            _ => Self::Unsupported,
        }
    }
}

// Attitude Euler Block 5938
#[binrw]
#[derive(Debug)]
pub struct AttEuler {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x)})]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x)})]
    pub wnc: Option<u16>,
    #[br(map = |x| if x == DO_NOT_USE_U1 { None } else { Some(x)})]
    pub nrsv: Option<u8>,
    // TODO: create Error enum
    pub error: u8,
    pub mode: u16,
    _reserved: u16,

    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub heading: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub pitch: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub roll: Option<f32>,

    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub pitch_dot: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub roll_dot: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub heading_dot: Option<f32>,
}

// INS Nav Geod Block 4226
#[binrw]
#[derive(Debug)]
pub struct INSNavGeod {
    #[br(map = |x| if x == DO_NOT_USE_U4 { None } else { Some(x)})]
    pub tow: Option<u32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x)})]
    pub wnc: Option<u16>,
    // TODO: create GNSSMode type for future telemetry info
    pub gnss_mode: u8,
    // TODO: create Error enum
    pub error: u8,
    // TODO: unpack this if we want more info
    pub info: u16,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x)})]
    pub gnss_age: Option<u16>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x)})]
    pub latitude: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x)})]
    pub longitude: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F8 { None } else { Some(x)})]
    pub height: Option<f64>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub undulation: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x)})]
    pub accuracy: Option<u16>,
    #[br(map = |x| if x == DO_NOT_USE_U2 { None } else { Some(x)})]
    pub latency: Option<u16>,
    // TODO: create a Datum enum
    #[br(map = |x| if x == DO_NOT_USE_U1 { None } else { Some(x)})]
    pub datum: Option<u8>,
    _reserved: u8,
    // TODO: unpack into an SBList type so we know what INSNav Sub Blocks we can parse
    pub sb_list: u16,

    // NOTE: Assuming that all sub blocks are populated by the message stream.
    // May want to change this in the future to be smarter

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
#[derive(Debug)]
pub struct INSNavGeodPosStdDev {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub longitude_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub latitude_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub height_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAtt {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub heading: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub pitch: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub roll: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAttStdDev {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub heading_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub pitch_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub roll_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVel {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub ve: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub vn: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub vu: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVelStdDev {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub ve_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub vn_std_dev: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub vu_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodPosCov {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub latitude_longitude_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub latitude_height_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub longitude_height_cov: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVelCov {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub ve_vn_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub ve_vu_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub vn_vu_cov: Option<f32>,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAttCov {
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub heading_pitch_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub heading_roll_cov: Option<f32>,
    #[br(map = |x| if x == DO_NOT_USE_F4 { None } else { Some(x)})]
    pub pitch_roll_cov: Option<f32>,
}

pub fn is_sync(bytes: &[u8; 2]) -> bool {
    bytes == b"$@"
}
