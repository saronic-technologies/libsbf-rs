use crate::binrw_util;
use binrw::binrw;

use super::att_euler::AttitudeMode;
use super::ins_nav_geod::{GnssMode, INSCouplingMode, INSError, INSSolutionLocation};
use super::pvt_geodetic::{Datum, PvtMode};

// INSNavCart Block 4225
#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCart {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    /// Bits 0-3: PVT mode, Bits 4-7: Attitude mode
    gnss_mode_raw: u8,
    pub error: u8,
    /// Bits 0-2: coupling mode, Bits 3-5: solution location, Bits 6-8: flags
    pub info: u16,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub gnss_age: Option<u16>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub x: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub y: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub z: Option<f64>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub accuracy: Option<u16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub latency: Option<u16>,
    #[br(map = binrw_util::map_datum)]
    #[bw(map = binrw_util::unmap_datum)]
    pub datum: Option<Datum>,
    _reserved: u8,
    pub sb_list: u16,

    #[br(if(sb_list & 1 == 1))]
    pub pos_std_dev: Option<INSNavCartPosStdDev>,
    #[br(if((sb_list >> 1) & 1 == 1))]
    pub att: Option<INSNavCartAtt>,
    #[br(if((sb_list >> 2) & 1 == 1))]
    pub att_std_dev: Option<INSNavCartAttStdDev>,
    #[br(if((sb_list >> 3) & 1 == 1))]
    pub vel: Option<INSNavCartVel>,
    #[br(if((sb_list >> 4) & 1 == 1))]
    pub vel_std_dev: Option<INSNavCartVelStdDev>,
    #[br(if((sb_list >> 5) & 1 == 1))]
    pub pos_cov: Option<INSNavCartPosCov>,
    #[br(if((sb_list >> 6) & 1 == 1))]
    pub att_cov: Option<INSNavCartAttCov>,
    #[br(if((sb_list >> 7) & 1 == 1))]
    pub vel_cov: Option<INSNavCartVelCov>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartPosStdDev {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub x_std_dev: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub y_std_dev: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub z_std_dev: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartAtt {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub heading: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub pitch: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub roll: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartAttStdDev {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub heading_std_dev: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub pitch_std_dev: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub roll_std_dev: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartVel {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vx: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vy: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vz: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartVelStdDev {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vx_std_dev: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vy_std_dev: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vz_std_dev: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartPosCov {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub xy_cov: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub xz_cov: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub yz_cov: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartVelCov {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vx_vy_cov: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vx_vz_cov: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vy_vz_cov: Option<f32>,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct INSNavCartAttCov {
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub heading_pitch_cov: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub heading_roll_cov: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub pitch_roll_cov: Option<f32>,
}

impl INSNavCart {
    /// Combined GNSS mode (PVT mode + attitude mode).
    pub fn gnss_mode(&self) -> GnssMode {
        GnssMode::from_byte(self.gnss_mode_raw)
    }

    /// Last GNSS PVT mode used by INS filter (bits 0-3 of gnss_mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.gnss_mode_raw & 0x0F)
    }

    /// Last GNSS Attitude mode used by INS filter (bits 4-7 of gnss_mode).
    pub fn attitude_mode(&self) -> AttitudeMode {
        AttitudeMode::from((self.gnss_mode_raw >> 4) as u16)
    }

    /// INS error code.
    pub fn ins_error(&self) -> INSError {
        INSError::from(self.error)
    }

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
