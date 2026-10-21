use alloc::vec::Vec;
use binrw::{binrw, io::SeekFrom, BinRead, BinResult, BinWrite};

// Constants for DO_NOT_USE values
const DO_NOT_USE_I1: i8 = -128;
const DO_NOT_USE_I2: i16 = -32768;
const DO_NOT_USE_U1: u8 = 255;
const DO_NOT_USE_U2: u16 = 65535;
const DO_NOT_USE_U4: u32 = 4294967295;
const DO_NOT_USE_F4: f32 = -2e10;
const DO_NOT_USE_F8: f64 = -2e10;

// Read: raw value to Option, None at the DO_NOT_USE value.
pub(crate) fn map_u1(x: u8) -> Option<u8> {
    if x == DO_NOT_USE_U1 { None } else { Some(x) }
}

pub(crate) fn map_u2(x: u16) -> Option<u16> {
    if x == DO_NOT_USE_U2 { None } else { Some(x) }
}

pub(crate) fn map_u4(x: u32) -> Option<u32> {
    if x == DO_NOT_USE_U4 { None } else { Some(x) }
}

pub(crate) fn map_i1(x: i8) -> Option<i8> {
    if x == DO_NOT_USE_I1 { None } else { Some(x) }
}

pub(crate) fn map_i2(x: i16) -> Option<i16> {
    if x == DO_NOT_USE_I2 { None } else { Some(x) }
}

pub(crate) fn map_f4(x: f32) -> Option<f32> {
    if x == DO_NOT_USE_F4 { None } else { Some(x) }
}

pub(crate) fn map_f8(x: f64) -> Option<f64> {
    if x == DO_NOT_USE_F8 { None } else { Some(x) }
}

// Write: Option back to raw, DO_NOT_USE value for None.
pub(crate) fn unmap_u1(x: &Option<u8>) -> u8 {
    x.unwrap_or(DO_NOT_USE_U1)
}

pub(crate) fn unmap_u2(x: &Option<u16>) -> u16 {
    x.unwrap_or(DO_NOT_USE_U2)
}

pub(crate) fn unmap_u4(x: &Option<u32>) -> u32 {
    x.unwrap_or(DO_NOT_USE_U4)
}

pub(crate) fn unmap_i1(x: &Option<i8>) -> i8 {
    x.unwrap_or(DO_NOT_USE_I1)
}

pub(crate) fn unmap_i2(x: &Option<i16>) -> i16 {
    x.unwrap_or(DO_NOT_USE_I2)
}

pub(crate) fn unmap_f4(x: &Option<f32>) -> f32 {
    x.unwrap_or(DO_NOT_USE_F4)
}

pub(crate) fn unmap_f8(x: &Option<f64>) -> f64 {
    x.unwrap_or(DO_NOT_USE_F8)
}

// Variants for fields that use 0 as the sentinel rather than a DO_NOT_USE value.
pub(crate) fn map_u1_zero(x: u8) -> Option<u8> {
    if x == 0 { None } else { Some(x) }
}

pub(crate) fn unmap_u1_zero(x: &Option<u8>) -> u8 {
    x.unwrap_or(0)
}

pub(crate) fn map_u2_zero(x: u16) -> Option<u16> {
    if x == 0 { None } else { Some(x) }
}

pub(crate) fn unmap_u2_zero(x: &Option<u16>) -> u16 {
    x.unwrap_or(0)
}

pub(crate) fn map_u4_zero(x: u32) -> Option<u32> {
    if x == 0 { None } else { Some(x) }
}

pub(crate) fn unmap_u4_zero(x: &Option<u32>) -> u32 {
    x.unwrap_or(0)
}

pub(crate) fn map_u64_zero(x: u64) -> Option<u64> {
    if x == 0 { None } else { Some(x) }
}

pub(crate) fn unmap_u64_zero(x: &Option<u64>) -> u64 {
    x.unwrap_or(0)
}

pub(crate) fn map_i1_zero(x: i8) -> Option<i8> {
    if x == 0 { None } else { Some(x) }
}

pub(crate) fn unmap_i1_zero(x: &Option<i8>) -> i8 {
    x.unwrap_or(0)
}

// Enum coded in a u1 field, None at the DO_NOT_USE value.
pub(crate) fn map_u1_enum_opt<T: From<u8>>(x: u8) -> Option<T> {
    if x == DO_NOT_USE_U1 { None } else { Some(T::from(x)) }
}

pub(crate) fn unmap_u1_enum_opt<T: Copy + Into<u8>>(x: &Option<T>) -> u8 {
    x.map_or(DO_NOT_USE_U1, Into::into)
}

/// Wrapper for a single SBF sub-block to allow padding of individual elements,
/// which is not supported on standalone structs.
#[binrw]
#[derive(Clone, Debug)]
#[brw(import(sb_len: usize))]
pub(crate) struct SubBlock<T>
where
    T: 'static,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    #[brw(pad_size_to = sb_len)]
    data: T,
}

impl<T> From<T> for SubBlock<T>
where
    T: 'static,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    fn from(data: T) -> Self {
        Self { data }
    }
}

impl<T> SubBlock<T>
where
    T: 'static,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    // Neither `From<SubBlock<T>> for T` (orphan rule) nor `Into<T> for
    // SubBlock<T>` (collides with core's blanket `Into`) is allowed, so the
    // unwrap direction is an inherent method.
    pub(crate) fn into_inner(self) -> T {
        self.data
    }
}

/// The count of nested second-level sub-blocks carried by a first-level header,
/// so [`NestedBlock`] can read that many without naming the count field.
pub(crate) trait NestedHeader {
    fn nested_count(&self) -> usize;
}

/// One first-level sub-block of a two-level SBF block: a fixed header padded to
/// SB1Length followed by the nested second-level sub-blocks, each padded to
/// SB2Length.
#[binrw]
#[derive(Clone, Debug)]
#[brw(import(sb1_len: usize, sb2_len: usize))]
pub(crate) struct NestedBlock<H, T>
where
    H: 'static,
    T: 'static,
    for<'a> H: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + NestedHeader + Clone,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + Clone,
{
    #[brw(pad_size_to = sb1_len)]
    pub header: H,
    #[br(args { count: header.nested_count(), inner: (sb2_len,) })]
    #[bw(args_raw = (sb2_len,))]
    pub items: Vec<SubBlock<T>>,
}

// Number of sub-blocks of size `sb_length` that fit into the remaining bytes
// such that the count modulo 256 is `n`.
#[binrw::parser(reader)]
pub(crate) fn subblock_count_mod256(n: u8, sb_length: u8) -> BinResult<usize> {
    let pos = reader.stream_position()?;
    // Length is not accessible here, but the slice we get has the exact length
    let end = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(pos))?;
    let remaining = usize::try_from(end - pos).unwrap_or(usize::MAX);
    let fit = remaining / usize::from(sb_length.max(1));
    let n = usize::from(n);
    Ok(fit.checked_sub(n).map_or(n, |extra| extra / 256 * 256 + n))
}

// Read `count` sub-blocks, each padded to its SBLength, unwrapped to the bare type.
pub(crate) fn unwrap_subblocks<T>(v: Vec<SubBlock<T>>) -> Vec<T>
where
    T: 'static,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    v.into_iter().map(SubBlock::into_inner).collect()
}

// Wrap the bare sub-blocks so each writes back padded to its SBLength. binrw's
// bw map passes the field as `&Vec<T>`, so the signature has to match it exactly.
#[allow(clippy::ptr_arg)]
pub(crate) fn wrap_subblocks<T>(v: &Vec<T>) -> Vec<SubBlock<T>>
where
    T: 'static + Clone,
    for<'a> T: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()>,
{
    v.iter().cloned().map(SubBlock::from).collect()
}
