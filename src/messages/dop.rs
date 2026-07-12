use binrw::binrw;

// DOP Block 4001
#[binrw]
#[derive(Clone, Debug)]
pub struct DOP {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    #[br(map = |x: u8| if x == 0 { None } else { Some(x) })]
    pub nr_sv: Option<u8>,
    pub reserved: u8,
    /// Position DOP * 100. Divide by 100 for actual PDOP.
    #[br(map = |x: u16| if x == 0 { None } else { Some(x) })]
    pub pdop: Option<u16>,
    /// Time DOP * 100.
    #[br(map = |x: u16| if x == 0 { None } else { Some(x) })]
    pub tdop: Option<u16>,
    /// Horizontal DOP * 100.
    #[br(map = |x: u16| if x == 0 { None } else { Some(x) })]
    pub hdop: Option<u16>,
    /// Vertical DOP * 100.
    #[br(map = |x: u16| if x == 0 { None } else { Some(x) })]
    pub vdop: Option<u16>,
    /// Horizontal Protection Level in meters.
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub hpl: Option<f32>,
    /// Vertical Protection Level in meters.
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
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
