use binrw::binrw;
use crate::do_not_use::{map_u1, map_u2, map_u4, map_f4, map_f8, unmap_u1, unmap_u2, unmap_u4, unmap_f4, unmap_f8};

// INS Nav Geod Block 4226
#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeod {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    // TODO: create GNSSMode type for future telemetry info
    pub gnss_mode: u8,
    // TODO: create Error enum
    pub error: u8,
    // TODO: unpack this if we want more info
    pub info: u16,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub gnss_age: Option<u16>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub latitude: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub longitude: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub height: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub undulation: Option<f32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub accuracy: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub latency: Option<u16>,
    // TODO: create a Datum enum
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub datum: Option<u8>,
    _reserved: u8,
    // TODO: unpack into an SBList type so we know what INSNav Sub Blocks we can parse
    pub sb_list: u16,

    #[br(if(sb_list & 1 == 1))]
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
#[derive(Debug, Clone)]
pub struct INSNavGeodPosStdDev {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub longitude_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub latitude_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub height_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeodAtt {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub roll: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeodAttStdDev {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub roll_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeodVel {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ve: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vn: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vu: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeodVelStdDev {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ve_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vn_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vu_std_dev: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeodPosCov {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub latitude_longitude_cov: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub latitude_height_cov: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub longitude_height_cov: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeodVelCov {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ve_vn_cov: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ve_vu_cov: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vn_vu_cov: Option<f32>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct INSNavGeodAttCov {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading_pitch_cov: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub heading_roll_cov: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub pitch_roll_cov: Option<f32>,
}