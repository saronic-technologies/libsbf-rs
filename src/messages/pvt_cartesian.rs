use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

use super::pvt_geodetic::{
    ArpOffset, Datum, DiffCorrType, PppSeedType, PvtError, PvtMode, PvtModeFlags, RaimIntegrity,
    WACorrFlags,
};

// PVTCartesian Block 4006
#[binrw]
#[derive(Clone, Debug)]
pub struct PVTCartesian {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    mode_raw: u8,
    #[br(map = |x: u8| PvtError::from(x))]
    #[bw(map = |x: &PvtError| u8::from(*x))]
    pub error: PvtError,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub x: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub y: Option<f64>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub z: Option<f64>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub undulation: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vx: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vy: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vz: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub cog: Option<f32>,
    #[br(map = binrw_util::map_f8)]
    #[bw(map = binrw_util::unmap_f8)]
    pub rx_clk_bias: Option<f64>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub rx_clk_drift: Option<f32>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub time_system: Option<u8>,
    #[br(map = binrw_util::map_datum)]
    #[bw(map = binrw_util::unmap_datum)]
    pub datum: Option<Datum>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub nr_sv: Option<u8>,
    wa_corr_info_raw: u8,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub reference_id: Option<u16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub mean_corr_age: Option<u16>,
    pub signal_info: u32,
    alert_flag_raw: u8,
    // Rev 1 fields
    pub nr_bases: u8,
    pub ppp_info: u16,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub latency: Option<u16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub h_accuracy: Option<u16>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub v_accuracy: Option<u16>,
    pub misc: u8,
    // Rev 2 fields
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl PVTCartesian {
    /// PVT mode (bits 0-3 of mode).
    pub fn pvt_mode(&self) -> PvtMode {
        PvtMode::from(self.mode_raw)
    }

    /// Mode flags (bits 6-7 of mode).
    pub fn mode_flags(&self) -> PvtModeFlags {
        PvtModeFlags::from_bits_truncate(self.mode_raw)
    }

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

    /// Age of the last PPP seed in seconds, from bits 0-11 of ppp_info,
    /// clipped to 4091. Ignore when the seed type is NotSeeded.
    pub fn ppp_seed_age(&self) -> u16 {
        self.ppp_info & 0x0FFF
    }

    /// Type of the last PPP seed, from bits 13-15 of ppp_info.
    pub fn ppp_seed_type(&self) -> PppSeedType {
        PppSeedType::from((self.ppp_info >> 13) as u8)
    }

    /// Bit 0 of misc: in DGNSS or RTK mode, the baseline points to the base
    /// station ARP rather than the antenna phase center.
    pub fn baseline_points_to_arp(&self) -> bool {
        self.misc & 1 != 0
    }

    /// Bit 1 of misc: the phase center offset is compensated for at the rover.
    pub fn phase_center_compensated(&self) -> bool {
        self.misc & (1 << 1) != 0
    }

    /// ARP-to-marker offset flag, from bits 6-7 of misc.
    pub fn arp_offset(&self) -> ArpOffset {
        ArpOffset::from(self.misc >> 6)
    }
}
