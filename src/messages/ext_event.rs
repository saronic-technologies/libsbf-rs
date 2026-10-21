use crate::util::binrw::{map_f4, map_f8, map_u2, map_u4, unmap_f4, unmap_f8, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;

// ExtEvent Block 5924
#[binrw]
#[derive(Clone, Debug)]
pub struct ExtEvent {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub source: u8,
    pub polarity: u8,
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub offset: Option<f32>,
    #[br(map = map_f8)]
    #[bw(map = unmap_f8)]
    pub rx_clk_bias: Option<f64>,
    // Rev 1
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
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
