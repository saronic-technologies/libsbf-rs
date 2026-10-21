use super::common::{Datum, GnssMode, INSCouplingMode, INSError, INSSolutionLocation};
use crate::util::binrw::{
    map_f4, map_f8, map_u1_enum, map_u1_enum_opt, map_u2, map_u4, unmap_f4, unmap_f8,
    unmap_u1_enum, unmap_u1_enum_opt, unmap_u2, unmap_u4,
};
use binrw::binrw;

// ExtEventINSNavGeod Block 4230
// Same structure as INSNavGeod, but at external event time.
#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEventINSNavGeod {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Last GNSS PVT mode and attitude mode used by the INS filter.
    #[br(map = GnssMode::from_byte)]
    #[bw(map = |x: &GnssMode| x.raw())]
    pub gnss_mode: GnssMode,
    /// INS error code.
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub error: INSError,
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
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub datum: Option<Datum>,
    _reserved: u8,
    pub sb_list: u16,

    #[br(if(sb_list & 1 == 1))]
    pub pos_std_dev: Option<ExtEventINSNavGeodPosStdDev>,
    #[br(if((sb_list >> 1) & 1 == 1))]
    pub att: Option<ExtEventINSNavGeodAtt>,
    #[br(if((sb_list >> 2) & 1 == 1))]
    pub att_std_dev: Option<ExtEventINSNavGeodAttStdDev>,
    #[br(if((sb_list >> 3) & 1 == 1))]
    pub vel: Option<ExtEventINSNavGeodVel>,
    #[br(if((sb_list >> 4) & 1 == 1))]
    pub vel_std_dev: Option<ExtEventINSNavGeodVelStdDev>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEventINSNavGeodPosStdDev {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub latitude_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub longitude_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub height_std_dev: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEventINSNavGeodAtt {
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
#[derive(Clone, Debug)]
pub struct ExtEventINSNavGeodAttStdDev {
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
#[derive(Clone, Debug)]
pub struct ExtEventINSNavGeodVel {
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
#[derive(Clone, Debug)]
pub struct ExtEventINSNavGeodVelStdDev {
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

impl ExtEventINSNavGeod {
    /// INS coupling mode (bits 0-2 of info).
    pub fn coupling_mode(&self) -> INSCouplingMode {
        INSCouplingMode::from(self.info)
    }

    /// Solution output location (bits 3-5 of info).
    pub fn solution_location(&self) -> INSSolutionLocation {
        INSSolutionLocation::from(self.info)
    }

    /// Bit 6: 180-degree heading ambiguity fixed.
    pub fn heading_ambiguity_fixed(&self) -> bool {
        self.info & (1 << 6) != 0
    }

    /// Bit 7: Zero-velocity constraints used.
    pub fn zero_velocity_constraints(&self) -> bool {
        self.info & (1 << 7) != 0
    }

    /// Bit 8: IMU orientation estimation has converged.
    pub fn imu_orientation_converged(&self) -> bool {
        self.info & (1 << 8) != 0
    }
}
