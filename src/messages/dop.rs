use crate::util::binrw::{
    map_f4, map_u1_zero, map_u2, map_u2_zero, map_u4, unmap_f4, unmap_u1_zero, unmap_u2,
    unmap_u2_zero, unmap_u4,
};
use binrw::binrw;

// DOP Block 4001
#[binrw]
#[derive(Clone, Debug)]
pub struct DOP {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = map_u1_zero)]
    #[bw(map = unmap_u1_zero)]
    pub nr_sv: Option<u8>,
    pub reserved: u8,
    /// Position DOP * 100. Divide by 100 for actual PDOP.
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub pdop: Option<u16>,
    /// Time DOP * 100.
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub tdop: Option<u16>,
    /// Horizontal DOP * 100.
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub hdop: Option<u16>,
    /// Vertical DOP * 100.
    #[br(map = map_u2_zero)]
    #[bw(map = unmap_u2_zero)]
    pub vdop: Option<u16>,
    /// Horizontal Protection Level in meters.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub hpl: Option<f32>,
    /// Vertical Protection Level in meters.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
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
