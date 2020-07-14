use crate::detail::fbapi::ibase as ib;

pub type Ptr<T> = *mut T;
pub type CPtr<T> = *const T;
pub type Void = std::ffi::c_void;
pub type VoidPtr = Ptr<Void>;
pub type VoidCPtr = CPtr<Void>;
pub type Int = libc::c_int;
pub type UInt = libc::c_uint;
pub type Char = libc::c_schar;
pub type UChar = libc::c_uchar;
pub type Long = libc::c_longlong;
pub type ULong = libc::c_ulonglong;
pub type IntPtr = libc::intptr_t;
pub type UIntPtr = libc::uintptr_t;

pub type IscStatus = IntPtr;
pub type FbBoolean = ib::FB_BOOLEAN;
pub type IscInt64 = ib::ISC_INT64;
pub type IscUInt64 = ib::ISC_UINT64;
pub type IscDate = ib::ISC_DATE;
pub type IscTime = ib::ISC_TIME;
pub type IscLong = ib::ISC_LONG;
pub type IscULong = ib::ISC_ULONG;
pub type GdsQuad = ib::GDS_QUAD_t;
pub type IscQuad = GdsQuad;

pub use crate::component::error::{Error, Result, NoRes};

