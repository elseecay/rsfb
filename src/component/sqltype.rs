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
use std::any::type_name;


pub type SqlTypeId = u32;

pub trait SqlType : Sized
{
    const TYPEID: SqlTypeId;

    fn from_buffer_nocheck(ptr: *const u8) -> Result<Self>;
    fn to_buffer_nocheck(&self, ptr: *mut u8) -> NoRes;

    fn from_buffer(ptr: *const u8, sqltype: SqlTypeId) -> Result<Self>
    {
        Self::check_typeid(sqltype);
        Self::from_buffer_nocheck(ptr)
    }
    fn to_buffer(&self, ptr: *mut u8, sqltype: SqlTypeId) -> NoRes
    {
        Self::check_typeid(sqltype);
        self.to_buffer_nocheck(ptr)
    }
    fn check_typeid(sqltype: SqlTypeId)
    {
        if Self::TYPEID != sqltype
        {
            panic!("Invalid SQL type {}, expected with typeid = {}", type_name::<Self>(), Self::TYPEID); // TODO: type name by type id
        }
    }
}

pub struct Varchar
{
    value: String
}

impl Varchar
{
    pub fn new(value: String) -> Varchar
    {
        return Varchar{ value };
    }
    pub fn value(&self) -> &str
    {
        return self.value.as_str();
    }
}

impl SqlType for Varchar
{
    const TYPEID: SqlTypeId = ib::SQL_VARYING;

    fn from_buffer_nocheck(ptr: *const u8) -> Result<Self>
    {
        unsafe
        {
            let vclen = from_raw_memory::<IscUShort>(ptr);
            let ptr = ptr.offset(size_of::<IscUShort>() as isize);
            let slc = slice_from_raw_parts(ptr, vclen as usize).as_ref().unwrap();
            let bytes = Vec::from(slc);
            let res = String::from_utf8(bytes);
            match res
            {
                Err(e) => Err(Error::from_str("Invalid UTF-8 string inside of buffer")),
                Ok(s) => Ok(Varchar::new(s))
            }
        }
    }
    fn to_buffer_nocheck(&self, ptr: *mut u8) -> NoRes
    {
        // TODO: put length
        let bytes = self.value.as_bytes();
        unsafe
        {
            libc::memcpy(ptr as VoidPtr, bytes.as_ptr() as VoidCPtr, bytes.len());
            *ptr.offset(bytes.len() as isize) = 0;
        }
        return Ok(());
    }
}

macro_rules! impl_simple_from_to
{
    () =>
    {
        fn from_buffer_nocheck(ptr: *const u8) -> Result<Self>
        {
            Ok(Self{ value: from_raw_memory(ptr) })
        }
        fn to_buffer_nocheck(&self, ptr: *mut u8) -> NoRes
        {
            to_raw_memory(ptr, self.value);
            Ok(())
        }
    }
}

macro_rules! impl_simple_type
{
    ($name: ident, $internal:ident, $id: ident) =>
    {
        pub struct $name
        {
            value: types::$internal
        }
        impl $name
        {
            pub fn new(value: types::$internal) -> $name
            {
                return $name{ value };
            }
            pub fn value(&self) -> types::$internal
            {
                return self.value;
            }
        }
        impl SqlType for $name
        {
            const TYPEID: SqlTypeId = ib::$id;

            impl_simple_from_to!();
        }
    }
}

impl_simple_type!(Short, IscShort, SQL_SHORT);
impl_simple_type!(Long, IscLong, SQL_LONG);
impl_simple_type!(Int64, IscInt64, SQL_INT64);
impl_simple_type!(Float, IscFloat, SQL_FLOAT);
impl_simple_type!(Double, IscDouble, SQL_DOUBLE);

// TODO: blobs
// TODO: charset?

