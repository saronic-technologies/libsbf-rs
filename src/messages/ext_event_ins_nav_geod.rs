use binrw::BinRead;

use super::att_euler::AttitudeMode;
use super::ins_nav_geod::{GnssMode, INSCouplingMode, INSError, INSSolutionLocation};
use super::pvt_geodetic::{Datum, PvtMode};

// ExtEventINSNavGeod Block 4230
// Same structure as INSNavGeod, but at external event time.
#[derive(Clone, Debug, BinRead)]
pub struct ExtEventINSNavGeod {
    #[br(map = |x| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    gnss_mode_raw: u8,
    pub error: u8,
    pub info: u16,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub gnss_age: Option<u16>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub latitude: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub longitude: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub height: Option<f64>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub undulation: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub accuracy: Option<u16>,
    #[br(map = |x| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub latency: Option<u16>,
    #[br(map = |x: u8| if x == crate::DO_NOT_USE_U1 { None } else { Some(Datum::from(x)) })]
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

#[derive(Clone, Debug, BinRead)]
pub struct ExtEventINSNavGeodPosStdDev {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub longitude_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub latitude_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub height_std_dev: Option<f32>,
}

#[derive(Clone, Debug, BinRead)]
pub struct ExtEventINSNavGeodAtt {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll: Option<f32>,
}

#[derive(Clone, Debug, BinRead)]
pub struct ExtEventINSNavGeodAttStdDev {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub heading_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub pitch_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub roll_std_dev: Option<f32>,
}

#[derive(Clone, Debug, BinRead)]
pub struct ExtEventINSNavGeodVel {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vu: Option<f32>,
}

#[derive(Clone, Debug, BinRead)]
pub struct ExtEventINSNavGeodVelStdDev {
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub ve_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vn_std_dev: Option<f32>,
    #[br(map = |x| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub vu_std_dev: Option<f32>,
}

impl ExtEventINSNavGeod {
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
