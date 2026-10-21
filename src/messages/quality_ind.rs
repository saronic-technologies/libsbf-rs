use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;
use core::fmt;
use num_enum::{FromPrimitive, IntoPrimitive};

// Indicator value the receiver reports when the quality is unknown.
const UNKNOWN_VALUE: u8 = 15;

/// Kind of quality indicator, from bits 0-7 of an indicator word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum QualityIndicatorType {
    /// Overall quality.
    OverallQuality = 0,
    /// GNSS signals from the main antenna.
    MainAntennaSignal = 1,
    /// GNSS signals from the aux1 antenna.
    AuxAntennaSignal = 2,
    /// RF power level from the main antenna.
    MainAntennaPower = 11,
    /// RF power level from the aux1 antenna.
    AuxAntennaPower = 12,
    /// CPU headroom.
    CpuHeadroom = 21,
    /// OCXO stability, available only on PolaRx5S receivers.
    OcxoStability = 25,
    /// Scintillation score, high when no ionospheric scintillation is detected.
    ScintillationScore = 29,
    /// Base station measurements, available only in RTK mode.
    BaseStationMeasurements = 30,
    /// Likelihood of a cm-accurate RTK position when post-processing the current
    /// data, available only when the position mode is not RTK.
    RtkPostProcessing = 31,
    /// Indicator type code not defined by the reference guide.
    #[num_enum(catch_all)]
    Unknown(u8),
}

/// One quality indicator word: its kind and its value.
#[binrw]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QualityIndicator(u16);

impl QualityIndicator {
    /// Kind of indicator, from bits 0-7.
    pub fn indicator_type(&self) -> QualityIndicatorType {
        QualityIndicatorType::from((self.0 & 0xFF) as u8)
    }

    /// Value from 0 for low quality to 10 for high quality, from bits 8-11. None when the
    /// receiver reports it as unknown.
    pub fn value(&self) -> Option<u8> {
        let value = ((self.0 >> 8) & 0x0F) as u8;
        (value != UNKNOWN_VALUE).then_some(value)
    }
}

impl fmt::Debug for QualityIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QualityIndicator")
            .field("indicator_type", &self.indicator_type())
            .field("value", &self.value())
            .finish()
    }
}

impl From<u16> for QualityIndicator {
    fn from(raw: u16) -> Self {
        Self(raw)
    }
}

impl From<QualityIndicator> for u16 {
    fn from(indicator: QualityIndicator) -> Self {
        indicator.0
    }
}

// Quality Indicator Block 4082
#[binrw]
#[derive(Clone, Debug)]
pub struct QualityInd {
    #[br(map = binrw_util::map_u4)]
    #[bw(map = binrw_util::unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = binrw_util::map_u2)]
    #[bw(map = binrw_util::unmap_u2)]
    pub wnc: Option<u16>,
    n: u8,
    reserved: u8,
    #[br(count = usize::from(n))]
    pub indicators: Vec<QualityIndicator>,
    #[br(parse_with = binrw::helpers::until_eof)]
    _padding: Vec<u8>,
}

impl PartialEq for QualityInd {
    fn eq(&self, other: &Self) -> bool {
        self.tow == other.tow
            && self.wnc == other.wnc
            && self.n == other.n
            && self.reserved == other.reserved
            && self.indicators == other.indicators
    }
}
