#[no_std]
use binrw::{binrw, BinRead, BinWrite};
use binrw::io::Cursor;

#[binrw]
#[derive(Debug)]
pub struct Id {
    pub bytes: u16
}

impl Id {
    pub fn message_type(&self) -> Messages {
        Messages::from(self.block_number())
    }

    pub fn block_number(&self) -> u16 {
        // NOTE: Bytes 0-12 are the actual Block Number
        self.bytes >> 3
    }

    pub fn block_rev_number(&self) -> u16 {
        // NOTE: Bytes 13-15 are the Block Revision Number
        self.bytes & 0x07
    }
}

#[binrw]
#[br(assert(length % 4 == 0, "length not a multiple of 4"))]
#[derive(Debug)]
pub struct Header {
    pub crc: u16,
    pub block_id: Id,
    pub length: u16
}

pub enum Messages {
    INSNavGeod,
    Unsupported,
}

impl From<u16> for Messages {
    fn from(block_number: u16) -> Self {
        match block_number {
            4226 => Self::INSNavGeod,
            _ => Self::Unsupported,
        }
    }
}

// INS Nav Geod Block 4226
#[binrw]
#[derive(Debug)]
pub struct INSNavGeod {
    pub tow: u32,
    pub wnc: u16,
    // TODO: create GNSSMode type for future telemetry info
    pub gnss_mode: u8,
    // TODO: create Error enum
    pub error: u8,
    // TODO: unpack this if we want more info
    pub info: u16,
    pub gnss_age: u16,
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
    pub undulation: f32,
    pub accuracy: u16,
    pub latency: u16,
    // TODO: create a Datum enum
    pub datum: u8,
    _reserved: u8,
    // TODO: unpack into an SBList type so we know what INSNav Sub Blocks we can parse
    pub sb_list: u16,

    // NOTE: Assuming that all sub blocks are populated by the message stream.
    // May want to change this in the future to be smarter

    pub vel_cov: INSNavGeodVelCov,
    pub att_cov: INSNavGeodAttCov,
    pub pos_cov: INSNavGeodPosCov,
    pub vel_std_dev: INSNavGeodVelStdDev,
    pub vel: INSNavGeodVel,
    pub att_std_dev: INSNavGeodAttStdDev,
    pub att: INSNavGeodAtt,
    pub pos_std_dev: INSNavGeodPosStdDev,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodPosStdDev {
    pub longitude_std_dev: f32,
    pub latitude_std_dev: f32,
    pub height_std_dev: f32,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAtt {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAttStdDev {
    pub heading_std_dev: f32,
    pub pitch_std_dev: f32,
    pub roll_std_dev: f32,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVel {
    pub ve: f32,
    pub vn: f32,
    pub vu: f32,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVelStdDev {
    pub ve_std_dev: f32,
    pub vn_std_dev: f32,
    pub vu_std_dev: f32,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodPosCov {
    pub latitude_longitude_cov: f32,
    pub latitude_height_cov: f32,
    pub longitude_height_cov: f32,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodVelCov {
    pub ve_vn_cov: f32,
    pub ve_vu_cov: f32,
    pub vn_vu_cov: f32,
}

#[binrw]
#[derive(Debug)]
pub struct INSNavGeodAttCov {
    pub heading_pitch_cov: f32,
    pub heading_roll_cov: f32,
    pub pitch_roll_cov: f32,
}


pub fn is_sync(bytes: &[u8; 2]) -> bool {
    bytes == b"$@"
}
