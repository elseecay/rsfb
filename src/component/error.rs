use crate::detail::util::share::*;

use crate::detail::fbapi::*;
use crate::detail::fbapi as fb;
use crate::detail::fbapi::ibase as ib;

type SqlCode = IscLong;

pub struct FbError
{
    status: fb::Status
}

impl FbError
{
    fn sqlcode(&self) -> Option<SqlCode>
    {
        let sqlcode = unsafe { ib::isc_sqlcode(self.status.get_errors()) }; // CHECK
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
        fb::Master::get().get_util_interface().format_status(bufptr, BUF_SIZE as UInt, &self.status);
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
    pub fn from_str(s: &str) -> Error
    {
        return Error::Api(ApiError{ text: s.to_owned() });
    }
    pub fn from_sw(sw: &fb::StatusWrapper) -> Error
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
pub type NoRes = Result<()>;
