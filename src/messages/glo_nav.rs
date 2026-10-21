use crate::util::{
    binrw::{map_u2, map_u4, unmap_u2, unmap_u4},
    signal::glonass_frequency_number,
};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// GLONav Block 4004
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct GLONav {
    /// SIS time stamp, in 0.001 s.
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    /// SIS time stamp, in weeks.
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    /// ID of the GLONASS satellite for which ephemeris is provided in this block.
    pub svid: u8,
    /// Frequency number of the GLONASS satellite for which ephemeris is provided in this
    /// block, with an offset of 8. For example, a value of 1 corresponds to frequency
    /// number -7.
    pub freq_nr: u8,
    /// x-component of satellite position in PZ-90, in 1000 m.
    pub x: f64,
    /// y-component of satellite position in PZ-90, in 1000 m.
    pub y: f64,
    /// z-component of satellite position in PZ-90, in 1000 m.
    pub z: f64,
    /// x-component of satellite velocity in PZ-90, in 1000 m/s.
    pub dx: f32,
    /// y-component of satellite velocity in PZ-90, in 1000 m/s.
    pub dy: f32,
    /// z-component of satellite velocity in PZ-90, in 1000 m/s.
    pub dz: f32,
    /// x-component of satellite acceleration in PZ-90, in 1000 m/s².
    pub ddx: f32,
    /// y-component of satellite acceleration in PZ-90, in 1000 m/s².
    pub ddy: f32,
    /// z-component of satellite acceleration in PZ-90, in 1000 m/s².
    pub ddz: f32,
    /// γn(tb): relative deviation of predicted carrier frequency, in Hz/Hz.
    pub gamma: f32,
    /// τn(tb): time correction to GLONASS time, in seconds.
    pub tau: f32,
    /// Δτn: time difference between L2 and L1 sub-band, in seconds.
    pub dtau: f32,
    /// Reference time-of-week in GPS time frame, in seconds.
    pub t_oe: u32,
    /// Reference week number in GPS time frame, modulo 1024.
    pub wn_toe: u16,
    /// Time interval between adjacent values of tb, in minutes.
    pub p1: u8,
    /// 1-bit odd/even flag of tb.
    pub p2: u8,
    /// Age of data, in days.
    pub e: u8,
    /// 3-bit health flag, satellite unhealthy if MSB set.
    pub b: u8,
    /// Time of day, the center of the validity interval, in minutes.
    pub tb: u16,
    /// 2-bit GLONASS-M satellite identifier, 01 for GLONASS-M and 00 otherwise.
    pub m: u8,
    /// 2-bit mode of computation of time parameters.
    pub p: u8,
    /// 1-bit health flag, 0 healthy and 1 unhealthy.
    pub l: u8,
    /// 1-bit 'updated' flag of ephemeris data.
    pub p4: u8,
    /// Current day number within the 4-year interval, in days.
    pub n_t: u16,
    /// Predicted user range accuracy at time tb, in 0.01 m.
    pub f_t: u16,
    /// 1-bit health Cn flag from the almanac, set to the last 'Cn' flag received from the
    /// almanac. It reads 255 if no 'Cn' flag has been received yet, and None for revision 0
    /// blocks, which do not carry it.
    #[br(try)]
    pub c: Option<u8>,
    /// Padding bytes.
    #[br(parse_with = binrw::helpers::until_eof)]
    pub padding: Vec<u8>,
}

impl GLONav {
    /// Frequency number of the satellite, from -7 to 6 for satellites launched after 2005 and
    /// up to 13 before that (supported in firmware prior to 1.5.x).
    pub fn frequency_number(&self) -> i8 {
        glonass_frequency_number(self.freq_nr)
    }
}
