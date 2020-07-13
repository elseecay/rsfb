#![allow(warnings)]

mod fbapi;
mod detail;
mod types;
mod pbuilder;

use fbapi::*;
use fbapi::ibase as ib;
use detail as dt;
use types::*;
use std::mem::MaybeUninit;
use std::ffi::CString;
use std::ffi::CStr;
use std::fmt;
use pbuilder as pb;
use pb::XpbParamsBuilder;


type SqlCode = IscLong;


pub struct FbError
{
    status: Status
}

impl FbError
{
    fn sqlcode(&self) -> Option<SqlCode>
    {
        let sqlcode = unsafe { fbapi::ib::isc_sqlcode(self.status.get_errors()) }; // CHECK
        match sqlcode
        {
            -999 => None,
            _ => Some(sqlcode)
        }
    }
    fn text(&self) -> String
    {
        const BUF_SIZE: usize = 750;
        let mut buf = MaybeUninit::<[Char; BUF_SIZE]>::uninit();
        let bufptr = buf.as_mut_ptr().cast::<Char>();
        Master::get().get_util_interface().format_status(bufptr, BUF_SIZE as UInt, &self.status);
        unsafe
        {
            match *bufptr
            {
                0 => "No errors".to_owned(), // CHECK
                _ => CStr::from_ptr(bufptr).to_str().unwrap().to_owned()
            }
        }
    }
}

pub struct ApiError
{
    text: String
}

impl ApiError
{
    fn sqlcode(&self) -> Option<SqlCode>
    {
        None
    }
    fn text(&self) -> String
    {
        self.text.clone()
    }
}

// TODO: rework to Box<dyn..>
pub enum Error
{
    Fb(FbError),
    Api(ApiError)
}

impl Error
{
    fn from_str(s: &str) -> Error
    {
        return Error::Api(ApiError{ text: s.to_owned() });
    }
    fn from_sw(sw: &StatusWrapper) -> Error
    {
        return Error::Fb(FbError{ status: sw.clone() });
    }
    pub fn sqlcode(&self) -> Option<SqlCode>
    {
        match self
        {
            Error::Fb(e) => e.sqlcode(),
            Error::Api(e) => e.sqlcode()
        }
    }
    pub fn text(&self) -> String
    {
        match self
        {
            Error::Fb(e) => e.text(),
            Error::Api(e) => e.text()
        }
    }
}

impl fmt::Debug for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("rsfb::Error").field("DESCRIPTION", &self.text()).finish()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Connection
{
    a: Attachment
}



impl Connection
{
    pub fn create_database<S: Into<Vec<u8>>>(filename: S, params: pb::CreateDatabase) -> NoRes
    {
        let m = Master::get();
        let s = m.get_status();
        let p = m.get_dispatcher();
        let u = m.get_util_interface();
        let sw = StatusWrapper::new(&s);
        let buf_len = params.get_buffer_length()?;
        let buf = params.get_buffer()?;
        let a = p.create_database(&sw, CString::new(filename).unwrap().as_ptr(), buf_len, buf)?;
        a.detach(&sw)?;
        return Ok(());
    }
    pub fn connect<S: Into<Vec<u8>>>(filename: S, params: pb::Connect) -> Result<Connection>
    {
        let m = Master::get();
        let s = m.get_status();
        let p = m.get_dispatcher();
        let u = m.get_util_interface();
        let sw = StatusWrapper::new(&s);
        let buf_len = params.get_buffer_length()?;
        let buf = params.get_buffer()?;
        let a = p.attach_database(&sw, CString::new(filename).unwrap().as_ptr(), buf_len, buf)?;
        return Ok(Connection{ a });
    }

    // pub fn execute<S: Into<Vec<u8>>>(&self, query: S, ) -> Result<u64>
    // {
    //
    // }
    //
    // pub fn start_transaction(params: pb::StartTransaction)
}


#[cfg(test)]
mod test;