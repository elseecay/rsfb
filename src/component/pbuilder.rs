use crate::detail::util::share::*;

use crate::detail::fbapi::*;
use crate::detail::fbapi as fb;
use crate::detail::fbapi::ibase as ib;

use crate::detail::util::*;



type Tag = UChar;

pub trait InsertTag
{
    fn insert_tag(&mut self, t: Tag) -> NoRes;
}
pub trait InsertInt
{
    fn insert_int(&mut self, t: Tag, v: Int) -> NoRes;
}
pub trait InsertLong
{
    fn insert_long(&mut self, t: Tag, v: Long) -> NoRes;
}
pub trait InsertStr
{
    fn insert_str<S: Into<Vec<u8>>>(&mut self, t: Tag, v: S) -> NoRes;
}
pub trait XpbParamsBuilder : InsertTag + InsertInt + InsertLong + InsertStr
{
    fn get_buffer(&self) -> Result<CPtr<UChar>>;
    fn get_buffer_length(&self) -> Result<UInt>;
}

pub mod params
{
    use super::*;

    macro_rules! declare_param_tag
    {
        ($name: ident, $tag: ident, $method: ident) =>
        {
            pub trait $name : InsertTag
            {
                fn $method(&mut self) -> NoRes
                {
                    self.insert_tag(ib::$tag as Tag)
                }
            }
        };
    }
    macro_rules! declare_param_int
    {
        ($name: ident, $tag: ident, $method: ident) =>
        {
            pub trait $name : InsertInt
            {
                fn $method(&mut self, v: Int) -> NoRes
                {
                    self.insert_int(ib::$tag as Tag, v)
                }
            }
        };
    }
    macro_rules! declare_param_long
    {
        ($name: ident, $tag: ident, $method: ident) =>
        {
            pub trait $name : InsertLong
            {
                fn $method(&mut self, v: Long) -> NoRes
                {
                    self.insert_long(ib::$tag as Tag, v)
                }
            }
        };
    }
    macro_rules! declare_param_str
    {
        ($name: ident, $tag: ident, $method: ident) =>
        {
            pub trait $name : InsertStr
            {
                fn $method<S: Into<Vec<u8>>>(&mut self, v: S) -> NoRes
                {
                    self.insert_str(ib::$tag as Tag, v)
                }
            }
        };
    }

    declare_param_tag!(Read, isc_tpb_read, read);
    declare_param_tag!(Write, isc_tpb_write, write);
    declare_param_tag!(ReadCommitted, isc_tpb_read_committed, read_committed);
    declare_param_tag!(AutoCommit, isc_tpb_autocommit, auto_commit);
    // #define isc_tpb_version1                  1
    // #define isc_tpb_version3                  3
    // #define isc_tpb_consistency               1
    // #define isc_tpb_concurrency               2
    // #define isc_tpb_shared                    3
    // #define isc_tpb_protected                 4
    // #define isc_tpb_exclusive                 5
    // #define isc_tpb_wait                      6
    // #define isc_tpb_nowait                    7
    // #define isc_tpb_lock_read                 10
    // #define isc_tpb_lock_write                11
    // #define isc_tpb_verb_time                 12
    // #define isc_tpb_commit_time               13
    // #define isc_tpb_ignore_limbo              14
    // #define isc_tpb_rec_version               17
    // #define isc_tpb_no_rec_version            18
    // #define isc_tpb_restart_requests          19
    // #define isc_tpb_no_auto_undo              20
    // #define isc_tpb_lock_timeout              21

    declare_param_int!(PageSize, isc_dpb_page_size, page_size);
    declare_param_str!(User, isc_dpb_user_name, user);
    declare_param_str!(Password, isc_dpb_password, password);
}

macro_rules! impl_xpb_param_builder
{
    ($name: ident, $kind: ident) =>
    {
        pub struct $name
        {
            builder: XpbBuilder,
            s: StatusWrapper
        }
        impl InsertTag for $name
        {
            fn insert_tag(&mut self, t: Tag) -> NoRes
            {
                self.builder.insert_tag(&self.s, t)
            }
        }
        impl InsertInt for $name
        {
            fn insert_int(&mut self, t: Tag, v: Int) -> NoRes
            {
                self.builder.insert_int(&self.s, t, v)
            }
        }
        impl InsertLong for $name
        {
            fn insert_long(&mut self, t: Tag, v: Long) -> NoRes
            {
                self.builder.insert_big_int(&self.s, t, v)
            }
        }
        impl InsertStr for $name
        {
            fn insert_str<S: Into<Vec<u8>>>(&mut self, t: Tag, v: S) -> NoRes
            {
                self.builder.insert_string(&self.s, t, CString::new(v).unwrap().as_ptr())
            }
        }
        impl XpbParamsBuilder for $name
        {
            fn get_buffer(&self) -> Result<CPtr<UChar>>
            {
                self.builder.get_buffer(&self.s)
            }
            fn get_buffer_length(&self) -> Result<UInt>
            {
                self.builder.get_buffer_length(&self.s)
            }
        }
        impl $name
        {
            pub fn new() -> Result<$name>
            {
                let m = Master::get();
                let s = create_status_wrapper();
                let builder = m.get_util_interface().get_xpb_builder(&s, XpbBuilder::$kind, null(), 0)?;
                return Ok($name{ builder, s });
            }
        }
    }
}

macro_rules! impl_param
{
    ($pname: ident, $bname: ident) =>
    {
        impl params::$pname for $bname { }
    }
}

impl_xpb_param_builder!(CreateDatabase, DPB);
impl_param!(PageSize, CreateDatabase);
impl_param!(User, CreateDatabase);
impl_param!(Password, CreateDatabase);

impl_xpb_param_builder!(Connect, DPB);
impl_param!(User, Connect);
impl_param!(Password, Connect);

impl_xpb_param_builder!(Transaction, TPB);
impl_param!(Read, Transaction);
impl_param!(Write, Transaction);
impl_param!(ReadCommitted, Transaction);
impl_param!(AutoCommit, Transaction);





