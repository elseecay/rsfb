pub mod stdimport;
pub mod types;
pub mod share;

use super::fbapi::*;
use std::ptr::null;


pub fn from_raw_memory<T: Copy>(ptr: *const u8) -> T
{
    unsafe { *(ptr as *const T) }
}

pub fn from_raw_memory_with_offset<T: Copy>(ptr: *const u8, offset: isize) -> T
{
    unsafe { *(ptr.offset(offset) as *const T) }
}

pub fn to_raw_memory<T: Copy>(ptr: *mut u8, value: T)
{
    unsafe { *(ptr as *mut T) = value };
}

pub fn to_raw_memory_with_offset<T: Copy>(ptr: *mut u8, offset: isize, value: T)
{
    unsafe { *(ptr.offset(offset) as *mut T) = value };
}

pub fn create_status_wrapper() -> StatusWrapper
{
    let m = Master::get();
    let s = m.get_status();
    return StatusWrapper::new(s);
}

pub fn check_allocation<T>(ptr: *const T)
{
    if ptr == null()
    {
        panic!("Allocation fails, no memory");
    }
}

