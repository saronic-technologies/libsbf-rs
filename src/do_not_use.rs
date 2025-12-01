/// Helper functions for DO_NOT_USE value conversions
use binrw::{BinResult, BinWrite, io::{Write, Seek}};

// Constants for DO_NOT_USE values
pub(crate) const DO_NOT_USE_I2: i16 = -32768;
pub(crate) const DO_NOT_USE_U1: u8  = 255;
pub(crate) const DO_NOT_USE_U2: u16 = 65535;
pub(crate) const DO_NOT_USE_U4: u32 = 4294967295;
pub(crate) const DO_NOT_USE_F4: f32 = -2e10;
pub(crate) const DO_NOT_USE_F8: f64 = -2e10;

// Read functions (map Option<T> from T)
pub fn map_u4(x: u32) -> Option<u32> {
    if x == DO_NOT_USE_U4 { None } else { Some(x) }
}

pub fn map_u2(x: u16) -> Option<u16> {
    if x == DO_NOT_USE_U2 { None } else { Some(x) }
}

pub fn map_u1(x: u8) -> Option<u8> {
    if x == DO_NOT_USE_U1 { None } else { Some(x) }
}

pub fn map_i2(x: i16) -> Option<i16> {
    if x == DO_NOT_USE_I2 { None } else { Some(x) }
}

pub fn map_f4(x: f32) -> Option<f32> {
    if x == DO_NOT_USE_F4 { None } else { Some(x) }
}

pub fn map_f8(x: f64) -> Option<f64> {
    if x == DO_NOT_USE_F8 { None } else { Some(x) }
}

// Write functions (unmap Option<T> to T)
pub fn unmap_u4(x: &Option<u32>) -> u32 {
    x.unwrap_or(DO_NOT_USE_U4)
}

pub fn unmap_u2(x: &Option<u16>) -> u16 {
    x.unwrap_or(DO_NOT_USE_U2)
}

pub fn unmap_u1(x: &Option<u8>) -> u8 {
    x.unwrap_or(DO_NOT_USE_U1)
}

pub fn unmap_i2(x: &Option<i16>) -> i16 {
    x.unwrap_or(DO_NOT_USE_I2)
}

pub fn unmap_f4(x: &Option<f32>) -> f32 {
    x.unwrap_or(DO_NOT_USE_F4)
}

pub fn unmap_f8(x: &Option<f64>) -> f64 {
    x.unwrap_or(DO_NOT_USE_F8)
}

// Writer function for Vec<T> where T: BinWrite
pub fn write_vec<T, W>(vec: &alloc::vec::Vec<T>, writer: &mut W, endian: binrw::Endian, _args: ()) -> BinResult<()>
where
    T: BinWrite<Args<'static> = ()>,
    W: Write + Seek,
{
    for item in vec {
        item.write_options(writer, endian, ())?;
    }
    Ok(())
}