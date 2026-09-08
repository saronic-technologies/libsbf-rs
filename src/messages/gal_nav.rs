use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

// GALNav Block 4002
#[binrw]
#[derive(Clone, Debug)]
pub struct GALNav {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    pub svid: u8,
    pub source: u8,
    pub sqrt_a: f64,
    pub m_0: f64,
    pub e: f64,
    pub i_0: f64,
    pub omega: f64,
    pub omega_0: f64,
    pub omegadot: f32,
    pub idot: f32,
    pub del_n: f32,
    pub c_uc: f32,
    pub c_us: f32,
    pub c_rc: f32,
    pub c_rs: f32,
    pub c_ic: f32,
    pub c_is: f32,
    pub t_oe: u32,
    pub t_oc: u32,
    pub a_f2: f32,
    pub a_f1: f32,
    pub a_f0: f64,
    pub wn_t_oe: u16,
    pub wn_t_oc: u16,
    pub iod_nav: u16,
    pub health_ossol: u16,
    pub health_prs: u8,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub sisa_l1e5a: Option<u8>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub sisa_l1e5b: Option<u8>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub sisa_l1ae6a: Option<u8>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub bgd_l1e5a: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub bgd_l1e5b: Option<f32>,
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub bgd_l1ae6a: Option<f32>,
    #[br(map = binrw_util::map_u1)]
    #[bw(map = binrw_util::unmap_u1)]
    pub cnav_enc: Option<u8>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

/// Health of one Galileo signal from the Health_OSSOL bit field: the 1-bit
/// Data Validity Status and 2-bit Health Status defined in the Galileo
/// Signal-In-Space ICD.
#[derive(Clone, Copy, Debug)]
pub struct GalSignalHealth {
    /// DVS bit: the signal is working without guarantee.
    pub working_without_guarantee: bool,
    /// HS code: 0 OK, 1 out of service, 2 will be out of service, 3 in test.
    pub health_status: u8,
}

impl GALNav {
    // Source constants
    pub const SOURCE_INAV: u8 = 2; // I/NAV (L1,E5b)
    pub const SOURCE_FNAV: u8 = 16; // F/NAV (L1,E5a)

    // CNAVenc bit masks
    pub const CNAV_E6B_UNENCRYPTED: u8 = 0x01;
    pub const CNAV_E6C_UNENCRYPTED: u8 = 0x02;

    fn signal_health(&self, shift: u8) -> Option<GalSignalHealth> {
        let bits = self.health_ossol >> shift;
        if bits & 1 == 0 {
            return None;
        }
        Some(GalSignalHealth {
            working_without_guarantee: bits & 0x02 != 0,
            health_status: ((bits >> 2) & 0x03) as u8,
        })
    }

    /// L1-B signal health, None when bit 0 of health_ossol marks it invalid.
    pub fn l1b_health(&self) -> Option<GalSignalHealth> {
        self.signal_health(0)
    }

    /// E5b signal health, None when bit 4 of health_ossol marks it invalid.
    pub fn e5b_health(&self) -> Option<GalSignalHealth> {
        self.signal_health(4)
    }

    /// E5a signal health, None when bit 8 of health_ossol marks it invalid.
    pub fn e5a_health(&self) -> Option<GalSignalHealth> {
        self.signal_health(8)
    }
}
