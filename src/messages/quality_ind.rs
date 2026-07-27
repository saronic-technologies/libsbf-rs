use crate::binrw_util;
use alloc::vec::Vec;
use binrw::binrw;

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

impl From<QualityIndicator> for u16 {
    fn from(indicator: QualityIndicator) -> Self {
        let (indicator_type, value): (u8, u8) = match indicator {
            QualityIndicator::OverallQuality(value) => (0, value),
            QualityIndicator::MainAntennaSignal(value) => (1, value),
            QualityIndicator::AuxAntennaSignal(value) => (2, value),
            QualityIndicator::MainAntennaPower(value) => (11, value),
            QualityIndicator::AuxAntennaPower(value) => (12, value),
            QualityIndicator::CpuHeadroom(value) => (21, value),
            QualityIndicator::OcxoStability(value) => (25, value),
            QualityIndicator::ScintillationScore(value) => (29, value),
            QualityIndicator::BaseStationMeasurements(value) => (30, value),
            QualityIndicator::RtkPostProcessing(value) => (31, value),
            QualityIndicator::Unknown { indicator_type, value } => (indicator_type, value),
        };
        u16::from(indicator_type) | (u16::from(value & 0x0F) << 8)
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
    #[br(count = usize::from(n), map = |v: Vec<u16>| v.into_iter().map(QualityIndicator::from).collect())]
    #[bw(map = |v: &Vec<QualityIndicator>| v.iter().map(|q| u16::from(*q)).collect::<Vec<u16>>())]
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
