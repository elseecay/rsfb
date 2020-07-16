#![allow(warnings)]

#[cfg(test)]
mod test;

pub mod component;
use component::pbuilder::*;
use component::pbuilder::params::*;
use component::pbuilder as pb;
use component::sqltype::*;

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

pub struct Statement<'a, 'b>
{
    t: &'a Transaction<'b>, // TODO: mb remove
    s: fb::Statement,
    imd: fb::MessageMetadata,
    omd: fb::MessageMetadata
}

struct FieldInfo
{
    pub sqltype: SqlTypeId,
    pub offset: isize,
    pub null_offset: isize,
    pub is_nullable: bool
}

pub struct Rows<'a, 'b>
{
    t: &'a Transaction<'b>, // TODO: mb remove
    sw: fb::StatusWrapper,
    rs: fb::ResultSet,
    field_info: Vec<FieldInfo>,
    message: Vec<u8>,
}

impl Rows<'_, '_>
{
    fn fetch_next(&mut self) -> Result<i32> // TODO: return something other
    {
        self.rs.fetch_next(&self.sw, self.message.as_mut_ptr() as Ptr<Void>)
    }
    fn get<T: SqlType>(&self, index: usize) -> Result<Option<T>> // TODO: something without .unwrap().unwrap().value()
    {
        let field_info = &self.field_info[index];
        unsafe
        {
            if *(self.message.as_ptr().offset(field_info.null_offset as isize) as CPtr<IscShort>) == ib::SQL_NULL as IscShort
            {
                return Ok(None);
            }
        }
        let ptr = unsafe { self.message.as_ptr().offset(field_info.offset) };
        return Ok(Some(T::from_buffer(ptr, field_info.sqltype)?));
    }
}

impl Transaction<'_>
{
    pub fn commit(self) -> NoRes
    {
        self.t.commit(&create_status_wrapper())
    }
    pub fn prepare<S: Into<Vec<u8>>>(&self, query: S) -> Result<Statement>
    {
        let s = create_status_wrapper();
        let bytes = query.into();
        let len = bytes.len() as UInt;
        let cstr = CString::new(bytes).unwrap();
        let stmt = self.c.a.prepare(&s, &self.t, len, cstr.as_ptr(), ib::SQL_DIALECT_CURRENT, fb::Statement::PREPARE_PREFETCH_METADATA)?;
        let imd = stmt.get_input_metadata(&s)?;
        let omd = stmt.get_output_metadata(&s)?;
        return Ok(Statement{ t: &self, s: stmt, imd, omd });
    }
    pub fn execute_prepared(&self, stmt: &Statement) -> Result<u64>
    {
        let sw = create_status_wrapper();
        stmt.s.execute(&sw, &self.t, &stmt.imd, null(), &stmt.omd, null())?;
        return Ok(stmt.s.get_affected_records(&sw)?);
    }
    pub fn execute_prepared_rows(&self, stmt: &Statement) -> Result<Rows>
    {
        let sw = create_status_wrapper();
        let cols = stmt.omd.get_count(&sw)?;
        let message_length = stmt.omd.get_message_length(&sw)?;
        let message = Vec::<u8>::with_capacity(message_length as usize);
        let mut field_info = Vec::<FieldInfo>::with_capacity(cols as usize);
        for i in 0..cols
        {
            let sqltype = stmt.omd.get_type(&sw, i)?;
            let offset = stmt.omd.get_offset(&sw, i)? as isize;
            let null_offset = stmt.omd.get_null_offset(&sw, i)? as isize;
            let is_nullable = match stmt.omd.is_nullable(&sw, i)? { 0 => false, _ => true };
            field_info.push(FieldInfo{ sqltype, offset, null_offset, is_nullable });
        }
        let rs = stmt.s.open_cursor(&sw, &self.t, &stmt.imd, null(), &stmt.omd, 0)?;
        return Ok(Rows{ t: &self, sw, rs, field_info, message });
    }
}

impl Connection
{
    pub fn create_database<S: Into<Vec<u8>>>(filename: S, params: pb::CreateDatabase) -> Result<Connection>
    {
        let m = Master::get();
        let s = m.get_status();
        let p = m.get_dispatcher();
        let u = m.get_util_interface();
        let sw = fb::StatusWrapper::new(s);
        let buf_len = params.get_buffer_length()?;
        let buf = params.get_buffer()?;
        let a = p.create_database(&sw, CString::new(filename).unwrap().as_ptr(), buf_len, buf)?;
        return Ok(Connection{ a });
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
}


