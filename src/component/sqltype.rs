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

pub trait SqlCheckNull
{
    fn is_null(&self) -> bool;
}

pub trait SqlInput : SqlGetTypeId + SqlCheckNull
{
    fn input(&self, dst: *mut u8) -> NoRes;
}

pub trait SqlOutput where Self: Sized
{
    const TYPEID: SqlTypeId;
    fn output(src: *const u8) -> Result<Self>;
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

macro_rules! impl_check_null
{
    ($name: ty) =>
    {
        impl SqlCheckNull for $name
        {
            fn is_null(&self) -> bool
            {
                return false;
            }
        }
    }
}



pub struct Null;

impl SqlCheckNull for Null
{
    fn is_null(&self) -> bool
    {
        return true;
    }
}

impl SqlGetTypeId for Null
{
    fn typeid(&self) -> SqlTypeId
    {
        panic!("It's a bug, please report");
    }
}

impl SqlInput for Null
{
    fn input(&self, dst: *mut u8) -> NoRes
    {
        panic!("It's a bug, please report");
    }
}


pub type Varchar = Vec<u8>;
impl_get_type_id!(Varchar);
impl_check_null!(Varchar);

impl SqlInput for Varchar
{
    fn input(&self, dst: *mut u8) -> NoRes
    {
        let bytes = self.as_slice();
        to_raw_memory(dst, bytes.len() as IscUShort);
        let dst = unsafe { dst.offset(size_of::<IscUShort>() as isize) };
        unsafe { libc::memcpy(dst as VoidPtr, bytes.as_ptr() as VoidCPtr, bytes.len()) };
        // CHECK: no needed *ptr.offset(bytes.len() as isize) = 0;
        return Ok(());
    }
}

impl SqlOutput for Varchar
{
    const TYPEID: SqlTypeId = ib::SQL_VARYING;

    fn output(src: *const u8) -> Result<Self>
    {
        let vclen = from_raw_memory::<IscUShort>(src);
        let src = unsafe { src.offset(size_of::<IscUShort>() as isize) };
        let slc = unsafe { slice_from_raw_parts(src, vclen as usize).as_ref().unwrap() };
        return Ok(Varchar::from(slc));
    }
}






























macro_rules! impl_simple_type
{
    ($name: ty, $id: expr) =>
    {
        impl_get_type_id!($name);
        impl_check_null!($name);
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

pub type Smallint = i16;
pub type Integer = i32;
pub type Bigint = i64;
pub type Float = f32;
pub type Double = f64;

impl_simple_type!(Smallint, ib::SQL_SHORT);
impl_simple_type!(Integer, ib::SQL_LONG);
impl_simple_type!(Bigint, ib::SQL_INT64);
impl_simple_type!(Float, ib::SQL_FLOAT);
impl_simple_type!(Double, ib::SQL_DOUBLE);


pub type InputSqlParams = &'static [&'static dyn SqlInput];

pub const NULL: &Null = &Null{};
pub const IN_EMPTY: InputSqlParams = &[];














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















