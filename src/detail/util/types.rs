use crate::detail::fbapi::ibase as ib;

pub type Ptr<T> = *mut T;
pub type CPtr<T> = *const T;
pub type Void = std::ffi::c_void;
pub type VoidPtr = Ptr<Void>;
pub type VoidCPtr = CPtr<Void>;
pub type Int = libc::c_int; // TODO: remove and find + replace to Isc*
pub type UInt = libc::c_uint;
pub type Char = libc::c_schar;
pub type UChar = libc::c_uchar;
pub type Long = libc::c_longlong;
pub type ULong = libc::c_ulonglong;
pub type IntPtr = libc::intptr_t;
pub type UIntPtr = libc::uintptr_t;
pub type Float = libc::c_float;
pub type Double = libc::c_double;

pub type IscChar = ib::ISC_SCHAR;
pub type IscUChar = ib::ISC_UCHAR;
pub type IscShort = ib::ISC_SHORT;
pub type IscUShort = ib::ISC_USHORT;
pub type IscInt = libc::c_int;
pub type IscUInt = libc::c_uint;
pub type IscLong = ib::ISC_LONG;
pub type IscULong = ib::ISC_ULONG;
pub type IscInt64 = ib::ISC_INT64;
pub type IscUInt64 = ib::ISC_UINT64;
pub type IscFloat = libc::c_float;
pub type IscDouble = libc::c_double;

pub type IscStatus = IntPtr;
pub type FbBoolean = ib::FB_BOOLEAN;
pub type IscDate = ib::ISC_DATE;
pub type IscTime = ib::ISC_TIME;
pub type GdsQuad = ib::GDS_QUAD_t;
pub type IscQuad = GdsQuad;

pub use crate::component::error::{Error, Result, NoRes};

