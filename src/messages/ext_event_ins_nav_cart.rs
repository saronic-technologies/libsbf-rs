use super::common::{Datum, GnssMode, INSCouplingMode, INSError, INSSolutionLocation};
use crate::util::binrw::{
    map_f4, map_f8, map_u1_enum, map_u1_enum_opt, map_u2, map_u4, unmap_f4, unmap_f8,
    unmap_u1_enum, unmap_u1_enum_opt, unmap_u2, unmap_u4,
};
use binrw::binrw;

// ExtEventINSNavCart Block 4229
// Same structure as INSNavCart, but at external event time.
#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEventINSNavCart {
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
    pub x: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub y: Option<f64>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub z: Option<f64>,
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
    pub pos_std_dev: Option<ExtEventINSNavCartPosStdDev>,
    #[br(if((sb_list >> 1) & 1 == 1))]
    pub att: Option<ExtEventINSNavCartAtt>,
    #[br(if((sb_list >> 2) & 1 == 1))]
    pub att_std_dev: Option<ExtEventINSNavCartAttStdDev>,
    #[br(if((sb_list >> 3) & 1 == 1))]
    pub vel: Option<ExtEventINSNavCartVel>,
    #[br(if((sb_list >> 4) & 1 == 1))]
    pub vel_std_dev: Option<ExtEventINSNavCartVelStdDev>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEventINSNavCartPosStdDev {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub x_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub y_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub z_std_dev: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEventINSNavCartAtt {
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
pub struct ExtEventINSNavCartAttStdDev {
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
pub struct ExtEventINSNavCartVel {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vx: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vy: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vz: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEventINSNavCartVelStdDev {
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vx_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vy_std_dev: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vz_std_dev: Option<f32>,
}

impl ExtEventINSNavCart {
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
