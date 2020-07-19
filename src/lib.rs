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
    t: &'a Transaction<'b>,
    s: fb::Statement,
    imd: fb::MessageMetadata,
    omd: fb::MessageMetadata
}

struct FieldInfo
{
    pub typeid: SqlTypeId,
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
    input_message: Vec<u8>,
    output_message: Vec<u8>,
}

impl Rows<'_, '_>
{
    fn fetch_next(&mut self) -> Result<i32> // TODO: return something other
    {
        self.rs.fetch_next(&self.sw, self.output_message.as_mut_ptr() as Ptr<Void>)
    }
    fn get<T: SqlOutput>(&self, index: usize) -> Result<Option<T>> // TODO: something without .unwrap().unwrap()
    {
        let field_info = &self.field_info[index];
        if field_info.typeid != T::TYPEID
        {
            panic!("Invalid SQL type {}, expected with typeid = {}", type_name::<T>(), field_info.typeid); // TODO: type name by type id
        }
        unsafe
        {
            if *(self.output_message.as_ptr().offset(field_info.null_offset as isize) as CPtr<IscShort>) == ib::SQL_NULL as IscShort
            {
                return Ok(None);
            }
        }
        let ptr = unsafe { self.output_message.as_ptr().offset(field_info.offset) };
        return Ok(Some(T::output(ptr)?));
    }
    fn get_null()
}

impl Transaction<'_>
{
    pub fn commit(self) -> NoRes
    {
        self.t.commit(&create_status_wrapper())
    }
    pub fn commit_retaining(&self) -> NoRes
    {
        self.t.commit_retaining(&create_status_wrapper())
    }
    pub fn rollback(self) -> NoRes
    {
        self.t.rollback(&create_status_wrapper())
    }
    pub fn rollback_retaining(&self) -> NoRes
    {
        self.t.rollback_retaining(&create_status_wrapper())
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
    pub fn execute_prepared(&self, stmt: &Statement, params: &[&dyn SqlInput]) -> Result<u64>
    {
        let sw = create_status_wrapper();
        let mut input_message = self.get_input_buffer(&sw, &stmt.imd, params)?;
        stmt.s.execute(&sw, &self.t, &stmt.imd, input_message.as_mut_ptr() as VoidPtr, &stmt.omd, null())?;
        return Ok(stmt.s.get_affected_records(&sw)?);
    }
    pub fn execute_prepared_rows(&self, stmt: &Statement, params: &[&dyn SqlInput]) -> Result<Rows>
    {
        let sw = create_status_wrapper();
        let mut input_message = self.get_input_buffer(&sw, &stmt.imd, params)?;
        let cols = stmt.omd.get_count(&sw)?;
        let output_message_length = stmt.omd.get_message_length(&sw)?;
        let output_message = Vec::<u8>::with_capacity(output_message_length as usize);
        let mut field_info = Vec::<FieldInfo>::with_capacity(cols as usize);
        for i in 0..cols
        {
            let sqltype = stmt.omd.get_type(&sw, i)?;
            let offset = stmt.omd.get_offset(&sw, i)? as isize;
            let null_offset = stmt.omd.get_null_offset(&sw, i)? as isize;
            let is_nullable = stmt.omd.is_nullable(&sw, i)? != 0;
            field_info.push(FieldInfo{ typeid: sqltype, offset, null_offset, is_nullable });
        }
        let rs = stmt.s.open_cursor(&sw, &self.t, &stmt.imd, input_message.as_mut_ptr() as VoidPtr, &stmt.omd, 0)?;
        return Ok(Rows{ t: &self, sw, rs, field_info, input_message, output_message });
    }
    fn get_input_buffer(&self, sw: &StatusWrapper, imd: &fb::MessageMetadata, params: &[&dyn SqlInput]) -> Result<Vec<u8>>
    {
        if imd.get_count(&sw)? as usize != params.len()
        {
            return Err(Error::from_str("Invalid number of input parameters")); // TODO: more info
        }
        let input_message_length = imd.get_message_length(&sw)?;
        let mut input_message = Vec::<u8>::with_capacity(input_message_length as usize);
        let input_base = input_message.as_mut_ptr();
        for i in 0..params.len()
        {
            if params[i].typeid() != imd.get_type(sw, i as UInt)?
            {
                return Err(Error::from_str("Invalid input parameter type")); // TODO: more info
            }
            let offset = imd.get_offset(&sw, i as UInt)? as isize;
            let dst = unsafe { input_base.offset(offset) };
            params[i].input(dst);
        }
        return Ok(input_message);
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
    pub fn disconnect(self) -> NoRes
    {
        self.a.detach(&create_status_wrapper())
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


