use crate::{Datum, SubBlock};
use alloc::vec::Vec;
use binrw::{BinRead, BinWrite};

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

pub(crate) fn map_i1_zero(x: i8) -> Option<i8> {
    if x == 0 { None } else { Some(x) }
}

pub(crate) fn unmap_i1_zero(x: &Option<i8>) -> i8 {
    x.unwrap_or(0)
}

// Enum fields stored as a raw u8 on the wire.
pub(crate) fn map_enum<T: From<u8>>(x: u8) -> T {
    T::from(x)
}

pub(crate) fn unmap_enum<T: Copy + Into<u8>>(x: &T) -> u8 {
    (*x).into()
}

// Datum, None at the DO_NOT_USE value.
pub(crate) fn map_datum(x: u8) -> Option<Datum> {
    if x == DO_NOT_USE_U1 { None } else { Some(Datum::from(x)) }
}

pub(crate) fn unmap_datum(x: &Option<Datum>) -> u8 {
    x.map_or(DO_NOT_USE_U1, u8::from)
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
