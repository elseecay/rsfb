// #define SQL_TEXT                           452
// #define SQL_VARYING                        448
// #define SQL_D_FLOAT                        530
// #define SQL_TIMESTAMP                      510
// #define SQL_BLOB                           520
// #define SQL_ARRAY                          540
// #define SQL_QUAD                           550
// #define SQL_TYPE_TIME                      560
// #define SQL_TYPE_DATE                      570
// #define SQL_BOOLEAN                      32764
// #define SQL_NULL                         32766

use crate::detail::util::share::*;
use crate::detail::util::types;
use crate::detail::util::{from_raw_memory, to_raw_memory, from_raw_memory_with_offset, to_raw_memory_with_offset};

use crate::detail::fbapi::ibase as ib;
use crate::component::error::ApiError;
use std::mem::size_of;
use std::ptr::slice_from_raw_parts;



pub type SqlTypeId = u32;

pub trait SqlGetTypeId
{
    fn typeid(&self) -> SqlTypeId;
}

macro_rules! impl_get_type_id
{
    ($name: ty) =>
    {
        impl SqlGetTypeId for $name
        {
            fn typeid(&self) -> SqlTypeId
            {
                <Self as SqlOutput>::TYPEID
            }
        }
    }
}

// TODO: input of NULL
pub trait SqlInput : SqlGetTypeId
{
    fn input(&self, dst: *mut u8) -> NoRes;
}

pub trait SqlOutput where Self: Sized
{
    const TYPEID: SqlTypeId;
    fn output(src: *const u8) -> Result<Self>;
}

// Varchar
impl SqlInput for Vec<u8>
{
    fn input(&self, dst: *mut u8) -> NoRes
    {
        let bytes = self.as_slice();
        to_raw_memory(dst, bytes.len() as IscUShort);
        let dst = unsafe { dst.offset(size_of::<IscUShort>() as isize) };
        unsafe { libc::memcpy(dst as VoidPtr, bytes.as_ptr() as VoidCPtr, bytes.len()) };
        // no needed *ptr.offset(bytes.len() as isize) = 0;
        return Ok(());
    }
}

impl SqlOutput for Vec<u8>
{
    const TYPEID: SqlTypeId = ib::SQL_VARYING;

    fn output(src: *const u8) -> Result<Self>
    {
        let vclen = from_raw_memory::<IscUShort>(src);
        let src = unsafe { src.offset(size_of::<IscUShort>() as isize) };
        let slc = unsafe { slice_from_raw_parts(src, vclen as usize).as_ref().unwrap() };
        return Ok(Vec::from(slc));
    }
}

impl_get_type_id!(Vec<u8>);



macro_rules! impl_simple_type
{
    ($name: ty, $id: expr) =>
    {
        impl_get_type_id!($name);
        impl SqlOutput for $name
        {
            const TYPEID: SqlTypeId = $id;

            fn output(src: *const u8) -> Result<Self>
            {
                Ok(from_raw_memory(src))
            }
        }
        impl SqlInput for $name
        {
            fn input(&self, dst: *mut u8) -> NoRes
            {
                to_raw_memory(dst, *self);
                Ok(())
            }
        }
    }
}

impl_simple_type!(i16, ib::SQL_SHORT);  // SMALLINT
impl_simple_type!(i32, ib::SQL_LONG);   // INTEGER
impl_simple_type!(i64, ib::SQL_INT64);  // BIGINT
impl_simple_type!(f32, ib::SQL_FLOAT);  // FLOAT
impl_simple_type!(f64, ib::SQL_DOUBLE); // DOUBLE PRECISION














// pub struct InputParamBuilder
// {
//     params: Vec<Box<dyn SqlInput>>
// }
//
// impl InputParamBuilder
// {
//     pub fn new() -> InputParamBuilder
//     {
//         InputParamBuilder{ params: Vec::new() }
//     }
//     pub fn push<T: SqlInput + 'static>(&mut self, value: T)
//     {
//         self.params.push(Box::new(value));
//     }
//     pub fn len(&self) -> usize
//     {
//         return self.params.len();
//     }
//     pub fn
// }


// TODO: blobs
// TODO: varchar with charset















