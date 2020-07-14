#![allow(warnings)]

#[cfg(test)]
mod test;

pub mod component;
use component::pbuilder::*;
use component::pbuilder::params::*;
use component::pbuilder as pb;

mod detail;
use detail::fbapi::*;
use detail::fbapi as fb;
use detail::fbapi::ibase as ib;
use detail::util::*;
use detail::util::share::*;



pub struct Connection
{
    a: fb::Attachment
}

pub struct Transaction<'a>
{
    c: &'a Connection,
    t: fb::Transaction
}

impl Transaction<'_>
{

}

impl Connection
{
    pub fn create_database<S: Into<Vec<u8>>>(filename: S, params: pb::CreateDatabase) -> NoRes
    {
        let m = Master::get();
        let s = m.get_status();
        let p = m.get_dispatcher();
        let u = m.get_util_interface();
        let sw = fb::StatusWrapper::new(s);
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
        let sw = fb::StatusWrapper::new(s);
        let buf_len = params.get_buffer_length()?;
        let buf = params.get_buffer()?;
        let a = p.attach_database(&sw, CString::new(filename).unwrap().as_ptr(), buf_len, buf)?;
        return Ok(Connection{ a });
    }
    pub fn transaction(&self, params: pb::Transaction) -> Result<Transaction>
    {
        let s = create_status_wrapper();
        let buf_len = params.get_buffer_length()?;
        let buf = params.get_buffer()?;
        let t = self.a.start_transaction(&s, buf_len, buf)?;
        return Ok(Transaction{ c: &self, t });
    }
    pub fn execute<S: Into<Vec<u8>>>(&self, query: S)
    {

    }
}


