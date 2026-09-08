use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use bitflags::bitflags;

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
    #[br(map = GALNavHealthOssol::from_bits_retain)]
    #[bw(map = |x: &GALNavHealthOssol| x.bits())]
    pub health_ossol: GALNavHealthOssol,
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

bitflags! {
    /// Health_OSSOL bit field of the [`GALNav`] block: the last received
    /// Health Status and Data Validity Status of the E5a, E5b and L1-B
    /// signals.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GALNavHealthOssol: u16 {
        /// Bit 0: the L1-B bits are valid.
        const L1B_VALID = 1 << 0;
        /// Bit 1: L1-B DVS, working without guarantee.
        const L1B_DVS = 1 << 1;
        /// Mask for the 2-bit L1-B HS code in bits 2-3, decoded by
        /// [`Self::l1b_health`].
        const L1B_HS_MASK = 0b11 << Self::L1B_HS_SHIFT;
        /// Bit 4: the E5b bits are valid.
        const E5B_VALID = 1 << 4;
        /// Bit 5: E5b DVS, working without guarantee.
        const E5B_DVS = 1 << 5;
        /// Mask for the 2-bit E5b HS code in bits 6-7, decoded by
        /// [`Self::e5b_health`].
        const E5B_HS_MASK = 0b11 << Self::E5B_HS_SHIFT;
        /// Bit 8: the E5a bits are valid.
        const E5A_VALID = 1 << 8;
        /// Bit 9: E5a DVS, working without guarantee.
        const E5A_DVS = 1 << 9;
        /// Mask for the 2-bit E5a HS code in bits 10-11, decoded by
        /// [`Self::e5a_health`].
        const E5A_HS_MASK = 0b11 << Self::E5A_HS_SHIFT;
    }
}

impl GALNavHealthOssol {
    const L1B_HS_SHIFT: u32 = 2;
    const E5B_HS_SHIFT: u32 = 6;
    const E5A_HS_SHIFT: u32 = 10;

    fn health(
        &self,
        valid: Self,
        dvs: Self,
        hs_mask: Self,
        hs_shift: u32,
    ) -> Option<GalSignalHealth> {
        if !self.contains(valid) {
            return None;
        }
        Some(GalSignalHealth {
            working_without_guarantee: self.contains(dvs),
            health_status: (self.intersection(hs_mask).bits() >> hs_shift) as u8,
        })
    }

    /// L1-B signal health, None when L1B_VALID is unset.
    pub fn l1b_health(&self) -> Option<GalSignalHealth> {
        self.health(
            Self::L1B_VALID,
            Self::L1B_DVS,
            Self::L1B_HS_MASK,
            Self::L1B_HS_SHIFT,
        )
    }

    /// E5b signal health, None when E5B_VALID is unset.
    pub fn e5b_health(&self) -> Option<GalSignalHealth> {
        self.health(
            Self::E5B_VALID,
            Self::E5B_DVS,
            Self::E5B_HS_MASK,
            Self::E5B_HS_SHIFT,
        )
    }

    /// E5a signal health, None when E5A_VALID is unset.
    pub fn e5a_health(&self) -> Option<GalSignalHealth> {
        self.health(
            Self::E5A_VALID,
            Self::E5A_DVS,
            Self::E5A_HS_MASK,
            Self::E5A_HS_SHIFT,
        )
    }
}

impl GALNav {
    // Source constants
    pub const SOURCE_INAV: u8 = 2; // I/NAV (L1,E5b)
    pub const SOURCE_FNAV: u8 = 16; // F/NAV (L1,E5a)

    // CNAVenc bit masks
    pub const CNAV_E6B_UNENCRYPTED: u8 = 0x01;
    pub const CNAV_E6C_UNENCRYPTED: u8 = 0x02;
}
