#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::boxed::Box;

// SBF uses the following type nomenclature:
mod primitive {
    #![allow(non_camel_case_types)]
    pub type u1 = core::primitive::u8;
    pub type u2 = core::primitive::u16;
    pub type u4 = core::primitive::u32;
    pub type u8 = core::primitive::u64;
    pub type i1 = core::primitive::i8;
    pub type i2 = core::primitive::i16;
    pub type i4 = core::primitive::i32;
    pub type i8 = core::primitive::i64;
    pub type f4 = core::primitive::f32;
    pub type f8 = core::primitive::f64;
    // TODO: Consider using ascii::Char if stabilized.
    pub type c1<const N: usize> = [u8; N];
}
use primitive::*;

// const DO_NOT_USE_I2: i16 = -32768;
// const DO_NOT_USE_U1: u8  = 255;
// const DO_NOT_USE_U2: u16 = 65535;
// const DO_NOT_USE_U4: u32 = 4294967295;
// const DO_NOT_USE_F4: f32 = -2e10;
// const DO_NOT_USE_F8: f64 = -2e10;

pub struct Block {
    pub header: Header,
    pub timestamp: Timestamp,
    pub data: Data,
}

/// SBF Block Header
///
/// Every SBF block starts with this 8-byte header.
#[derive(Debug)]
pub struct Header {
    pub crc: u2,
    pub id: u2,
    pub length: u2,
}

pub struct Timestamp {
    pub tow: u4,
    pub wnc: u2,
}

pub enum Data {
    // Measurement Blocks
    // Navigation Page Blocks
    // GPS Decoded Message Blocks
    // GLONASS Decoded Message Blocks
    // Galileo Decoded Message Blocks
    // BeiDou Decoded Message Blocks
    // QZSS Decoded Message Blocks
    // SBAS L1 Decoded Message Blocks
    // GNSS Position, Velocity, & Time Blocks
    // INS/GNSS Integreated Blocks
    // GNSS Attitude Blocks
    // Receiver Time Blocks
    // External Event Blocks
    // Differential Correction Blocks
    // External Sensor Blocks
    // Status Blocks
    // Miscellaneous Blocks
}

// Measurement Blocks
pub struct MeasEpoch {
    n1: u1,
    sb1_length: u1,
    sb2_length: u1,
    common_flags: u1,
    cum_clk_jumps: u1,
    channels: Box<[MeasEpochChannelType1]>,
}

pub struct MeasEpochChannelType1 {
    rx_channel: u1,
    signal_type: u1,
    svid: u1,
    misc: u1,
    code_lsb: u1,
    doppler: i4,
    carrier_lsb: u2,
    carrier_msb: i1,
    cn0: u1,
    lock_time: u2,
    obs_info: u1,
    n2: u1,
    channels: Box<[MeasEpochChannelType2]>,
}

pub struct MeasEpochChannelType2 {
    signal_type: u1,
    lock_time: u1,
    cn0: u1,
    offsets_msb: u1,
    carrier_msb: i1,
    obs_info: u1,
    code_offset_lsb: u2,
    carrier_lsb: u2,
    doppler_offset_lsb: u2,
}

pub struct MeasExtra {
    n: u1,
    sb_length: u1,
    doppler_var_factor: f4,
    channels: Box<[MeasExtraChannelSub]>,
}

pub struct MeasExtraChannelSub {
    ex_channel: u1,
    signal_type: u1,
    mp_correction: i2,
    smoothing_corr: i2,
    code_var: u2,
    carrier_var: u2,
    lock_time: u2,
    cum_loss_cont: u1,
    car_mp_corr: i1,
    info: u1,
    misc: u1,
}

// TODO: Contact Septentrio support for the Meas3* block definitions.
pub struct Meas3Ranges;
pub struct Meas3CN0HiRes;
pub struct Meas3Doppler;
pub struct Meas3Pp;
pub struct Meas3Mp;

pub struct EndOfMeas;           // Intentionally Empty.

// Navigation Page Blocks
pub struct NavRaw<const N: usize> {
    svid: u1,
    crc_passed: u1,
    viterbi_cnt: u1,
    source: u1,
    freq_nr: u1,
    rx_channel: u1,
    nav_bits: [u4; N],
}

pub type GpsRawCA = NavRaw<10>;
pub type GpsRawL2C = NavRaw<10>;
pub type GpsRawL5 = NavRaw<10>;
pub type GloRawCA = NavRaw<3>;
pub type GalRawFNav = NavRaw<8>;
pub type GalRawINav = NavRaw<8>;
pub type GeoRawL1 = NavRaw<8>;
pub type GeoRawL5 = NavRaw<8>;
pub type BdsRaw = NavRaw<10>;
pub type QzsRawL1CA = NavRaw<10>;
pub type QzsRawL2C = NavRaw<10>;
pub type QzsRawL5 = NavRaw<10>;

// GPS Decoded Message Blocks
#[allow(non_snake_case)]
pub struct GpsNav {
    pwn: u1,
    _reserved: u1,
    wn: u2,
    ca_or_pon_l2: u1,
    ura: u1,
    health: u1,
    l2_data_flag: u1,
    iodc: u2,
    iode2: u1,
    iode3: u1,
    fit_int_flag: u1,
    _reserved_2: u1,
    T_gd: f4,
    t_oc: u4,
    a_f2: f4,
    a_f1: f4,
    a_f0: f4,
    C_rs: f4,
    del_n: f4,
    M_0: f8,
    C_uc: f4,
    e: f8,
    C_us: f4,
    sqrt_A: f8,
    t_oe: u4,
    C_ic: f4,
    omega_0: f8,
    C_is: f4,
    i_0: f8,
    C_rc: f4,
    omega: f8,
    omega_dot: f4,
    wn_t_oc: u2,
    wn_t_oe: u2,
}

#[allow(non_snake_case)]
pub struct GpsAlm {
    prn: u1,
    reserved: u1,
    e: f4,
    t_oa: u4,
    delta_i: f4,
    omega_dot: f4,
    sqrt_A: f8,
    omega_0: f4,
    omega: f4,
    M_0: f4,
    a_f1: f4,
    a_f0: f4,
    wn_a: u1,
    config: u1,
    health_8: u1,
    health_6: u1,
}

#[allow(non_snake_case)]
pub struct GpsIon {
    prn: u1,
    reserved: u1,
    alpha: [f4; 4],
    beta: [f4; 4],
}

#[allow(non_snake_case)]
pub struct GpsUtc {
    prn: u1,
    reserved: u1,
    A_1: f4,
    A_0: f8,
    t_ot: u4,
    wn_t: u1,
    del_t_ls: i1,
    wn_lsf: u1,
    dn: u1,
    del_t_lsf: i1,
}

// GLONASS Decoded Message Blocks
#[allow(non_snake_case)]
pub struct GloNav {
    svid: u1,
    freq_nr: u1,
    x: f8,
    y: f8,
    z: f8,
    dx: f4,
    dy: f4,
    dz: f4,
    ddx: f4,
    ddy: f4,
    ddz: f4,
    gamma: f4,
    tau: f4,
    dtau: f4,
    t_oe: u4,
    wn_toe: u2,
    P1: u1,
    P2: u1,
    E: u1,
    B: u1,
    tb: u2,
    M: u1,
    P: u1,
    l: u1,
    P4: u1,
    N_T: u2,
    F_T: u2,
    C: u1,
}

#[allow(non_snake_case)]
pub struct GloAlm {
    svid: u1,
    freq_nr: u1,
    epsilon: f4,
    t_oa: u4,
    Delta_i: f4,
    lambda: f4,
    t_ln: f4,
    omega: f4,
    Delta_T: f4,
    dDelta_T: f4,
    tau: f4,
    wn_a: u1,
    C: u1,
    N: u2,
    M: u1,
    N_4: u1,
}

#[allow(non_snake_case)]
pub struct GloTime {
    svid: u1,
    freq_n: u1,
    n_4: u1,
    kp: u1,
    N: u2,
    tau_gps: f4,
    tau_c: f8,
    B1: f4,
    B2: f4,
}

// Galileo Decoded Message Blocks
#[allow(non_snake_case)]
pub struct GalNav {
    svid: u1,
    source: u1,
    sqrt_A: u1,
    M_0: f8,
    e: f8,
    i_0: f8,
    omega: f8,
    omega_0: f8,
    omega_dot: f4,
    idot: f4,
    del_N: f4,
    C_uc: f4,
    C_rc: f4,
    C_rs: f4,
    C_ic: f4,
    C_is: f4,
    t_oe: u4,
    a_f2: f4,
    a_f1: f4,
    wn_t_oe: u2,
    wn_t_oc: u2,
    iod_nav: u2,
    health_ossol: u2,
    health_prs: u1,
    sisa_l1e5a: u1,
    sisa_l1e5b: u1,
    sisa_l1ae6a: u1,
    bgd_l1e5a: f4,
    bgd_l1e5b: f4,
    cnav_enc: u1,
}

#[allow(non_snake_case)]
pub struct GalAlm {
    svid: u1,
    source: u1,
    e: f4,
    t_oa: u4,
    delta_i: f4,
    omega_dot: f4,
    sqrt_A: f4,
    omega_0: f4,
    omega: f4,
    M_0: f4,
    a_f1: f4,
    a_f0: f4,
    wn_a: u1,
    svid_a: u1,
    health: u2,
    ioda: u1,
}

#[allow(non_snake_case)]
pub struct GalIon {
    svid: u1,
    source: u1,
    a_i: [f4; 3],
    storm_flags: u1,
}

#[allow(non_snake_case)]
pub struct GalUtc {
    svid: u1,
    source: u1,
    A_1: f4,
    A_0: f8,
    t_ot: u4,
    wn_ot: u1,
    del_t_ls: i1,
    wn_lsf: u1,
    dn: u1,
    del_t_lsf: i1,
}

#[allow(non_snake_case)]
pub struct GalGstGps {
    svid: u1,
    source: u1,
    A_1G: f4,
    A_0G: f4,
    t_oG: u4,
    wn_oG: u1,
}

#[allow(non_snake_case)]
pub struct GalSarRlm<const N: usize> {
    svid: u1,
    source: u1,
    rlm_length: u1,
    _reserved: [u1; 3],
    rlm_bits: [u4; N],
}

// BeiDou Decoded Message Blocks
#[allow(non_snake_case)]
pub struct BdsNav {
    prn: u1,
    _reserved: u1,
    wn: u2,
    ura: u1,
    sat_h1: u1,
    iodc: u1,
    iode: u1,
    _reserved_2: u2,
    t_gd1: f4,
    t_gd2: f4,
    t_oc: u4,
    a_f2: f4,
    a_f1: f4,
    a_f0: f4,
    C_rs: f4,
    del_n: f4,
    M_0: f8,
    C_uc: f4,
    e: f8,
    C_us: f4,
    sqrt_A: f8,
    t_oe: u4,
    C_ic: f4,
    omega_0: f8,
    C_is: f4,
    i_0: f8,
    C_rc: f4,
    omega: f8,
    omega_dot: f4,
    idot: f4,
    wn_t_oc: u2,
    wn_t_oe: u2,
}

#[allow(non_snake_case)]
pub struct BdsAlm {
    prn: u1,
    wn_a: u1,
    t_oa: u4,
    sqrt_A: f4,
    e: f4,
    omega: f4,
    M_0: f4,
    omega_0: f4,
    omega_dot: f4,
    delta_i: f4,
    a_f0: f4,
    a_f1: f4,
    health: u2,
    _reserved: [u1; 2],
}

#[allow(non_snake_case)]
pub struct BdsIon {
    prn: u1,
    _reserved: u1,
    alpha: [f4; 4],
    beta: [f4; 4],
}

#[allow(non_snake_case)]
pub struct BdsUtc {
    prn: u1,
    _reserved: u1,
    A_1: f4,
    A_0: f8,
    del_t_ls: i1,
    wn_lsf: u1,
    dn: u1,
    del_t_lsf: i1,
}

// QZSS Decoded Message Blocks
#[allow(non_snake_case)]
pub struct QzsNav {
    prn: u1,
    _reserved: u1,
    wn: u2,
    ca_or_pon_l2: u1,
    ura: u1,
    health: u1,
    l2_data_flag: u1,
    iodc: u2,
    iode2: u1,
    iode3: u1,
    fit_int_flag: u1,
    _reserved_2: u1,
    T_gd: f4,
    t_oc: u4,
    a_f2: f4,
    a_f1: f4,
    a_f0: f4,
    C_rs: f4,
    del_n: f4,
    M_0: f8,
    C_uc: f4,
    e: f8,
    C_us: f4,
    sqrt_A: f8,
    t_oe: u4,
    C_ic: f4,
    omega_0: f8,
    C_is: f4,
    i_0: f8,
    C_rc: f4,
    omega: f8,
    omega_dot: f4,
    idot: f4,
    wn_t_oc: u2,
    wn_t_oe: u2,
}

#[allow(non_snake_case)]
pub struct QzsAlm {
    prn: u1,
    _reserved: u1,
    e: f4,
    t_oa: u4,
    delta_i: f4,
    omega_dot: f4,
    sqrt_A: f4,
    omega: f4,
    M_0: f4,
    a_f1: f4,
    a_f0: f4,
    wn_a: u1,
    _reserved_2: u1,
    health_8: u1,
    health_6: u1,
}

// SBAS L1 Decoded Message Blocks
pub struct GeoMt00 {
    prn: u1,
}

pub struct GeoPrnMask {
    prn: u1,
    iodp: u1,
    nbr_prns: u1,
    prn_mask: Box<[u1]>,
}

pub struct GeoFastCorr {
    prn: u1,
    mt: u1,
    iodp: u1,
    iodf: u1,
    n: u1,
    sb_length: u1,
    fast_corrs: Box<[FastCorr]>,
}

pub struct FastCorr {
    prn_mask_no: u1,
    udrei: u1,
    _reserved: [u1; 2],
    prc: f4,
}

pub struct GeoIntegrity {
    prn: u1,
    _reserved: u1,
    iodf: [u1; 4],
    udrei: [u1; 51],
}

pub struct GeoFastCorrDegr {
    prn: u1,
    iodp: u1,
    t_lat: u1,
    ai: [u1; 51],
}

#[allow(non_snake_case)]
pub struct GeoNav {
    prn: u1,
    _reserved: u1,
    iodn: u1,
    ura: u2,
    t0: u4,
    xg: f8,
    yg: f8,
    zg: f8,
    xgd: f8,
    ygd: f8,
    zgd: f8,
    xgdd: f8,
    ygdd: f8,
    zgdd: f8,
    aGf: [f4; 2],
}

#[allow(non_snake_case)]
pub struct GeoDegrFactors {
    prn: u1,
    _reserved: u1,
    Brrc: f8,
    C_ltc_lsb: f8,
    C_ltc_v1: f8,
    I_ltc_v1: u4,
    C_ltc_v0: f8,
    I_ltc_v0: u4,
    C_geo_lsb: f8,
    C_geo_v: f8,
    I_geo: u4,
    C_er: f4,
    C_iono_step: f8,
    I_iono: u4,
    C_iono_ramp: f8,
    rss_udre: u1,
    rss_iono: u1,
    _reserved_2: [u1; 2],
    C_covariance: f8,
}

#[allow(non_snake_case)]
pub struct GeoNetworkTime {
    prn: u1,
    _reserved: u1,
    A_1: f4,
    A_0: f8,
    t_ot: u4,
    wn_t: u1,
    del_t_ls: i1,
    wn_lsf: u1,
    dn: u1,
    del_t_lsf: i1,
    utc_std: u1,
    gps_wn: u2,
    gps_tow: u4,
    glonass_id: u1,
}

pub struct GeoAlm {
    prn: u1,
    _reserved_0: u1,
    data_id: u1,
    _reserved_1: u1,
    health: u2,
    t_oa: u4,
    xg: f8,
    yg: f8,
    zg: f8,
    xgd: f8,
    ygd: f8,
    zgd: f8,
}

pub struct GeoIgpMask {
    prn: u1,
    nbr_bands: u1,
    band_nbr: u1,
    iodi: u1,
    nbr_igps: u1,
    igp_mask: Box<[u1]>,
}

pub struct GeoLongTermCorr {
    prn: u1,
    n: u1,
    sb_length: u1,
    _reserved: [u1; 3],
    lt_corr: Box<[LtCorr]>,
}

pub struct LtCorr {
    velocity_code: u1,
    prn_mask_no: u1,
    iodp: u1,
    iode: u1,
    dx: f4,
    dy: f4,
    dz: f4,
    dx_rate: f4,
    dy_rate: f4,
    dz_rate: f4,
    da_f: [f4; 2],
    t_oe: u4,
}

pub struct GeoIonoDelay {
    prn: u1,
    band_nbr: u1,
    iodi: u1,
    n: u1,
    sb_length: u1,
    _reserved: u1,
    idc: Box<[Idc]>,
}

pub struct Idc {
    igp_mask_no: u1,
    givei: u1,
    _reserved: [u1; 2],
    vertical_delay: f4,
}

pub struct GeoServiceLevel {
    prn: u1,
    _reserved: u1,
    iods: u1,
    nr_messages: u1,
    priority_code: u1,
    dudre_in: u1,
    dudre_out: u1,
    n: u1,
    sb_length: u1,
    regions: Box<[ServiceRegion]>,
}

pub struct ServiceRegion {
    latitude1: i1,
    latitude2: i1,
    longitude1: i1,
    longitude2: i1,
    region_shape: u1
}

pub struct GeoClockEphCovMatrix {
    prn: u1,
    iodp: u1,
    n: u1,
    sb_length: u1,
    _reserved: [u1; 2],
    cov_matrix: Box<[CovMatrix]>,
}

pub struct CovMatrix {
    prn_mask_no: u1,
    _reserved: [u1; 2],
    scale_exp: u1,
    e11: u2,
    e12: i2,
    e13: i2,
    e14: i2,
    e22: u2,
    e23: i2,
    e24: i2,
    e33: u2,
    e34: i2,
    e44: u2,
}

// GNSS Position, Velocity, & Time Blocks
pub struct PvtCartesian {
    mode: u1,
    error: u1,
    x: f8,
    y: f8,
    z: f8,
    undulation: f4,
    vx: f4,
    vy: f4,
    vz: f4,
    cog: f4,
    rx_clock_bias: f8,
    rx_clock_drift: f4,
    time_system: u1,
    datum: u1,
    nrsv: u1,
    wa_corr_info: u1,
    reference_id: u2,
    mean_corr_age: u2,
    signal_info: u4,
    alert_flag: u1,
    nr_bases: u1,
    ppp_info: u2,
    latency: u2,
    h_accuracy: u2,
    v_accuracy: u2,
    misc: u1,
}
// INS/GNSS Integreated Blocks
// GNSS Attitude Blocks
// Receiver Time Blocks
// External Event Blocks
// Differential Correction Blocks
// External Sensor Blocks
// Status Blocks
// Miscellaneous Blocks
