use binrw::binrw;

// GEONav Block 5896
#[binrw]
#[derive(Debug)]
pub struct GEONav {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub prn: u8,
    pub reserved: u8,
    pub iodn: u16,
    pub ura: u16,
    pub t0: u32,
    pub xg: f64,
    pub yg: f64,
    pub zg: f64,
    pub xgd: f64,
    pub ygd: f64,
    pub zgd: f64,
    pub xgdd: f64,
    pub ygdd: f64,
    pub zgdd: f64,
    pub a_gf0: f32,
    pub a_gf1: f32,
}
