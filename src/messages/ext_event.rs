use alloc::vec::Vec;
use binrw::BinRead;

// ExtEvent Block 5924
#[derive(Debug, BinRead)]
pub struct ExtEvent {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub source: u8,
    pub polarity: u8,
    #[br(map = |x: f32| if x == crate::DO_NOT_USE_F4 { None } else { Some(x) })]
    pub offset: Option<f32>,
    #[br(map = |x: f64| if x == crate::DO_NOT_USE_F8 { None } else { Some(x) })]
    pub rx_clk_bias: Option<f64>,
    // Rev 1
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub pvt_age: Option<u16>,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

/// Event input pin source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSource {
    EventA,
    EventB,
    Unknown(u8),
}

impl From<u8> for EventSource {
    fn from(value: u8) -> Self {
        match value {
            1 => EventSource::EventA,
            2 => EventSource::EventB,
            x => EventSource::Unknown(x),
        }
    }
}

/// Event polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPolarity {
    Rising,
    Falling,
    Unknown(u8),
}

impl From<u8> for EventPolarity {
    fn from(value: u8) -> Self {
        match value {
            0 => EventPolarity::Rising,
            1 => EventPolarity::Falling,
            x => EventPolarity::Unknown(x),
        }
    }
}

impl ExtEvent {
    /// Event input pin.
    pub fn event_source(&self) -> EventSource {
        EventSource::from(self.source)
    }

    /// Event polarity (rising or falling edge).
    pub fn event_polarity(&self) -> EventPolarity {
        EventPolarity::from(self.polarity)
    }
}
