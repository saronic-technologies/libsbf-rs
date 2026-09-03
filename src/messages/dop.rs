use crate::binrw_util;
use binrw::binrw;
use serde::Serialize;

// DOP Block 4001
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct DOP {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = binrw_util::map_u1_zero)]
    #[bw(map = binrw_util::unmap_u1_zero)]
    pub nr_sv: Option<u8>,
    pub reserved: u8,
    /// Position DOP * 100. Divide by 100 for actual PDOP.
    #[br(map = binrw_util::map_u2_zero)]
    #[bw(map = binrw_util::unmap_u2_zero)]
    pub pdop: Option<u16>,
    /// Time DOP * 100.
    #[br(map = binrw_util::map_u2_zero)]
    #[bw(map = binrw_util::unmap_u2_zero)]
    pub tdop: Option<u16>,
    /// Horizontal DOP * 100.
    #[br(map = binrw_util::map_u2_zero)]
    #[bw(map = binrw_util::unmap_u2_zero)]
    pub hdop: Option<u16>,
    /// Vertical DOP * 100.
    #[br(map = binrw_util::map_u2_zero)]
    #[bw(map = binrw_util::unmap_u2_zero)]
    pub vdop: Option<u16>,
    /// Horizontal Protection Level in meters.
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub hpl: Option<f32>,
    /// Vertical Protection Level in meters.
    #[br(map = binrw_util::map_f4)]
    #[bw(map = binrw_util::unmap_f4)]
    pub vpl: Option<f32>,
}

impl DOP {
    /// Position DOP as f64.
    pub fn pdop_value(&self) -> Option<f64> {
        self.pdop.map(|v| f64::from(v) / 100.0)
    }

    /// Time DOP as f64.
    pub fn tdop_value(&self) -> Option<f64> {
        self.tdop.map(|v| f64::from(v) / 100.0)
    }

    /// Horizontal DOP as f64.
    pub fn hdop_value(&self) -> Option<f64> {
        self.hdop.map(|v| f64::from(v) / 100.0)
    }

    /// Vertical DOP as f64.
    pub fn vdop_value(&self) -> Option<f64> {
        self.vdop.map(|v| f64::from(v) / 100.0)
    }
}
