use crate::fbapi::*;
use crate::fbapi::ib;
use crate::types::*;
use std::ffi::CString;


type Tag = UChar;

trait InsertTag
{
    fn insert_tag(&mut self, t: Tag) -> NoRes;
}
trait InsertInt
{
    fn insert_int(&mut self, t: Tag, v: Int) -> NoRes;
}
trait InsertLong
{
    fn insert_long(&mut self, t: Tag, v: Long) -> NoRes;
}
trait InsertStr
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

    // TODO: declaring macro
    pub trait PageSize : InsertInt
    {
        fn set_page_size(&mut self, v: Int) -> NoRes
        {
            self.insert_int(ib::isc_dpb_page_size as Tag, v)
        }
    }
    pub trait User : InsertStr
    {
        fn set_user<S: Into<Vec<u8>>>(&mut self, v: S) -> NoRes
        {
            self.insert_str(ib::isc_dpb_user_name as Tag, v)
        }
    }
    pub trait Password : InsertStr
    {
        fn set_password<S: Into<Vec<u8>>>(&mut self, v: S) -> NoRes
        {
            self.insert_str(ib::isc_dpb_password as Tag, v)
        }
    }
    // tpb->insertTag(&status, isc_tpb_read_committed);
    // tpb->insertTag(&status, isc_tpb_no_rec_version);
    // tpb->insertTag(&status, isc_tpb_wait);
    // tpb->insertTag(&status, isc_tpb_read)
}

macro_rules! impl_xpb_param_builder
{
    ($name: ident, $kind: ident) =>
    {
        pub struct $name
        {
            builder: XpbBuilder,
            s: Status,
            sw: StatusWrapper
        }
        impl InsertTag for $name
        {
            fn insert_tag(&mut self, t: Tag) -> NoRes
            {
                self.builder.insert_tag(&self.sw, t)
            }
        }
        impl InsertInt for $name
        {
            fn insert_int(&mut self, t: Tag, v: Int) -> NoRes
            {
                self.builder.insert_int(&self.sw, t, v)
            }
        }
        impl InsertLong for $name
        {
            fn insert_long(&mut self, t: Tag, v: Long) -> NoRes
            {
                self.builder.insert_big_int(&self.sw, t, v)
            }
        }
        impl InsertStr for $name
        {
            fn insert_str<S: Into<Vec<u8>>>(&mut self, t: Tag, v: S) -> NoRes
            {
                self.builder.insert_string(&self.sw, t, CString::new(v).unwrap().as_ptr())
            }
        }
        impl XpbParamsBuilder for $name
        {
            fn get_buffer(&self) -> Result<CPtr<UChar>>
            {
                self.builder.get_buffer(&self.sw)
            }
            fn get_buffer_length(&self) -> Result<UInt>
            {
                self.builder.get_buffer_length(&self.sw)
            }
        }
        impl $name
        {
            pub fn new() -> Result<$name>
            {
                let m = Master::get();
                let s = m.get_status();
                let sw = StatusWrapper::new(&s);
                let builder = m.get_util_interface().get_xpb_builder(&sw, XpbBuilder::$kind, null(), 0)?;
                return Ok($name{ builder, s, sw });
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

impl_xpb_param_builder!(StartTransaction, TPB);



