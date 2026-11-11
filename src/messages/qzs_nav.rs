use crate::util::binrw::{map_f4, map_u2, map_u4, unmap_f4, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;

// QZSNav Block 4095
#[binrw]
#[derive(Clone, Debug)]
pub struct QZSNav {
    /// SIS time stamp, in 0.001 s.
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    /// SIS time stamp, in weeks.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// ID of the QZSS satellite of which the ephemeris is given in this block.
    pub prn: u8,
    /// Reserved for future use, to be ignored by decoding software.
    _reserved: u8,
    /// Week number, 10 bits from subframe 1, word 3.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wn: Option<u16>,
    /// Bit field containing bits 71 and 72 from subframe 1. Bit 0 is bit 72 of subframe 1,
    /// the Ephemeris Status Flag. Bit 1 is bit 71 of subframe 1, which is reserved. Bits 2-7
    /// are reserved. These bits were defined as "C/A or P on L2" in QZSS ICD is-qzss-pnt-004
    /// and earlier, and firmware prior to 1.5.x reports that older field, which is always 2
    /// for QZSS satellites.
    pub ca_or_p_on_l2: u8,
    /// User Range accuracy index, 4 bits from subframe 1, word 3.
    pub ura: u8,
    /// 6-bit health from subframe 1, word 3.
    pub health: u8,
    /// Data flag for L2 P-code, 1 bit from subframe 1, word 4. Always 1 for QZSS satellites.
    pub l2_data_flag: u8,
    /// Issue of data, clock, 10 bits from subframe 1.
    pub iodc: u16,
    /// Issue of data, ephemeris, 8 bits from subframe 2.
    pub iode2: u8,
    /// Issue of data, ephemeris, 8 bits from subframe 3.
    pub iode3: u8,
    /// Curve Fit Interval, 1 bit from subframe 2, word 10.
    pub fit_int_flg: u8,
    /// Unused, to be ignored by decoding software.
    _reserved2: u8,
    /// Estimated group delay differential, in seconds.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub t_gd: Option<f32>,
    /// Clock data reference time, in seconds.
    pub t_oc: u32,
    /// SV clock aging, in s/s².
    pub a_f2: f32,
    /// SV clock drift, in s/s.
    pub a_f1: f32,
    /// SV clock bias, in seconds.
    pub a_f0: f32,
    /// Amplitude of the sine harmonic correction term to the orbit radius, in meters.
    pub c_rs: f32,
    /// Mean motion difference from computed value, in semi-circles per second.
    pub del_n: f32,
    /// Mean anomaly at reference time, in semi-circles.
    pub m_0: f64,
    /// Amplitude of the cosine harmonic correction term to the argument of latitude, in
    /// radians.
    pub c_uc: f32,
    /// Eccentricity.
    pub e: f64,
    /// Amplitude of the sine harmonic correction term to the argument of latitude, in
    /// radians.
    pub c_us: f32,
    /// Square root of the semi-major axis, in m^1/2.
    pub sqrt_a: f64,
    /// Reference time ephemeris, in seconds.
    pub t_oe: u32,
    /// Amplitude of the cosine harmonic correction term to the angle of inclination, in
    /// radians.
    pub c_ic: f32,
    /// Longitude of ascending node of orbit plane at weekly epoch, in semi-circles.
    pub omega_0: f64,
    /// Amplitude of the sine harmonic correction term to the angle of inclination, in
    /// radians.
    pub c_is: f32,
    /// Inclination angle at reference time, in semi-circles.
    pub i_0: f64,
    /// Amplitude of the cosine harmonic correction term to the orbit radius, in meters.
    pub c_rc: f32,
    /// Argument of perigee, in semi-circles.
    pub omega: f64,
    /// Rate of right ascension, in semi-circles per second.
    pub omegadot: f32,
    /// Rate of inclination angle, in semi-circles per second.
    pub idot: f32,
    /// WN associated with t_oc, modulo 1024.
    pub wn_t_oc: u16,
    /// WN associated with t_oe, modulo 1024.
    pub wn_t_oe: u16,
    /// Padding bytes.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
