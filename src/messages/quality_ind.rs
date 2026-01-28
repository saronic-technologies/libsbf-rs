use alloc::vec::Vec;
use binrw::BinRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityIndicator {
    OverallQuality(u8),
    MainAntennaSignal(u8),
    AuxAntennaSignal(u8),
    MainAntennaPower(u8),
    AuxAntennaPower(u8),
    CpuHeadroom(u8),
    OcxoStability(u8),
    ScintillationScore(u8),
    BaseStationMeasurements(u8),
    RtkPostProcessing(u8),
    Unknown { indicator_type: u8, value: u8 },
}

impl From<u16> for QualityIndicator {
    fn from(raw: u16) -> Self {
        let indicator_type = (raw & 0xFF) as u8;
        let value = ((raw >> 8) & 0x0F) as u8;
        match indicator_type {
            0 => QualityIndicator::OverallQuality(value),
            1 => QualityIndicator::MainAntennaSignal(value),
            2 => QualityIndicator::AuxAntennaSignal(value),
            11 => QualityIndicator::MainAntennaPower(value),
            12 => QualityIndicator::AuxAntennaPower(value),
            21 => QualityIndicator::CpuHeadroom(value),
            25 => QualityIndicator::OcxoStability(value),
            29 => QualityIndicator::ScintillationScore(value),
            30 => QualityIndicator::BaseStationMeasurements(value),
            31 => QualityIndicator::RtkPostProcessing(value),
            _ => QualityIndicator::Unknown { indicator_type, value },
        }
    }
}

// Quality Indicator Block 4082
#[derive(Debug, BinRead)]
pub struct QualityInd {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    n: u8,
    reserved: u8,
    #[br(count = usize::from(n), map = |v: Vec<u16>| v.into_iter().map(QualityIndicator::from).collect())]
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
