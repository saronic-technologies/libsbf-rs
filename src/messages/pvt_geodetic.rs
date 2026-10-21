use super::common::{
    Datum, DiffCorrType, PppInfo, PvtError, PvtMisc, RaimIntegrity, RawPvtMode, TimeSystem,
    WACorrFlags,
};
use crate::util::binrw::{
    map_f4, map_f8, map_u1, map_u1_enum, map_u1_enum_opt, map_u1_zero, map_u2, map_u4, map_u4_zero,
    unmap_f4, unmap_f8, unmap_u1, unmap_u1_enum, unmap_u1_enum_opt, unmap_u1_zero, unmap_u2,
    unmap_u4, unmap_u4_zero,
};
use alloc::vec::Vec;
use binrw::binrw;

// PVTGeodetic Block 4007
#[binrw]
#[derive(Clone, Debug)]
pub struct PVTGeodetic {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// Bit field indicating the GNSS PVT mode.
    pub mode: RawPvtMode,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub error: PvtError,
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
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vn: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub ve: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub vu: Option<f32>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub cog: Option<f32>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub rx_clk_bias: Option<f64>,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub rx_clk_drift: Option<f32>,
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub time_system: Option<TimeSystem>,
    #[br(map = map_u1_enum_opt)]
    #[bw(map = unmap_u1_enum_opt)]
    pub datum: Option<Datum>,
    #[br(map = map_u1)]
    #[bw(map = unmap_u1)]
    pub nr_sv: Option<u8>,
    wa_corr_info_raw: u8,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub reference_id: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub mean_corr_age: Option<u16>,
    #[br(map = map_u4_zero)]
    #[bw(map = unmap_u4_zero)]
    pub signal_info: Option<u32>,
    alert_flag_raw: u8,
    // Rev 1 fields
    #[br(map = map_u1_zero)]
    #[bw(map = unmap_u1_zero)]
    pub nr_bases: Option<u8>,
    pub ppp_info: PppInfo,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub latency: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub h_accuracy: Option<u16>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub v_accuracy: Option<u16>,
    #[br(map = PvtMisc::from_bits_retain)]
    #[bw(map = |x: &PvtMisc| x.bits())]
    pub misc: PvtMisc,
    // Rev 2 fields
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl PVTGeodetic {
    /// Wide Area correction flags (bits 0-4).
    pub fn wa_corr_flags(&self) -> WACorrFlags {
        WACorrFlags::from_bits_truncate(self.wa_corr_info_raw)
    }

    /// Differential correction type (bits 5-6).
    pub fn diff_corr_type(&self) -> DiffCorrType {
        DiffCorrType::from(self.wa_corr_info_raw)
    }

    /// RAIM integrity status (bits 0-1 of alert_flag).
    pub fn raim_integrity(&self) -> RaimIntegrity {
        RaimIntegrity::from(self.alert_flag_raw)
    }

    /// Bit 2: Galileo HPCA integrity failed.
    pub fn galileo_hpca_failed(&self) -> bool {
        self.alert_flag_raw & (1 << 2) != 0
    }

    /// Bit 3: Galileo ionospheric storm active.
    pub fn galileo_iono_storm(&self) -> bool {
        self.alert_flag_raw & (1 << 3) != 0
    }

    /// Bit 5: GNSS signals may not be authentic, as flagged by SIG_AUTH_ALERT in RFStatus.
    pub fn signal_auth_alert(&self) -> bool {
        self.alert_flag_raw & (1 << 5) != 0
    }

    /// Bit 6: A navigation message failed authentication, as flagged by NAV_MSG_AUTH_ALERT
    /// in RFStatus.
    pub fn nav_msg_auth_alert(&self) -> bool {
        self.alert_flag_raw & (1 << 6) != 0
    }
}
