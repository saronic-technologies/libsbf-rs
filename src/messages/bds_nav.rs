use crate::util::binrw::{map_f4, map_u2, map_u4, unmap_f4, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// BDSNav Block 4081
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct BDSNav {
    /// SIS time stamp, in 0.001 s.
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    /// SIS time stamp, in weeks.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// ID of the BeiDou satellite of which the ephemeris is given in this block.
    pub prn: u8,
    /// Reserved for future use, to be ignored by decoding software.
    _reserved: u8,
    /// BeiDou week number as received from the navigation message, from 0 to 8191.
    pub wn: u16,
    /// User range accuracy index, a 4-bit value.
    pub ura: u8,
    /// 1-bit autonomous health.
    pub sat_h1: u8,
    /// Age of data, clock, a 5-bit value.
    pub iodc: u8,
    /// Age of data, ephemeris, a 5-bit value.
    pub iode: u8,
    /// Unused, to be ignored by decoding software.
    _reserved2: u16,
    /// B1I equipment group delay differential, in seconds.
    pub t_gd1: f32,
    /// B2I equipment group delay differential, in seconds. None when unknown.
    #[br(map = map_f4)]
    #[bw(map = unmap_f4)]
    pub t_gd2: Option<f32>,
    /// Clock data reference time in seconds, in BeiDou system time, which lags GPS time by
    /// 14 seconds.
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
    /// Reference time ephemeris in seconds, in BeiDou system time, which lags GPS time by 14
    /// seconds.
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
    /// BeiDou week number associated with t_oc, modulo 8192. This value relates to the
    /// BeiDou system time.
    pub wn_t_oc: u16,
    /// BeiDou week number associated with t_oe, modulo 8192. This value relates to the
    /// BeiDou system time.
    pub wn_t_oe: u16,
    /// Padding bytes.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}
