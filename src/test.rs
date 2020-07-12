use crate::fbapi::*;
use std::str;
use std::ffi::CStr;
use std::ptr::{null as nullptr, slice_from_raw_parts};
use std::process::exit;
use crate::types::*;
use crate::{Error, Result};
use std::ffi::CString;



#[test]
fn example_create_database()
{
    return;
    std::env::set_var("ISC_USER", "lck");
    std::env::set_var("ISC_PASSWORD", "666");

    let m = Master::get();
    let s = m.get_status();
    let sw = StatusWrapper::new(&s);
    // println!("1: {}", Error::from_sw(&sw).text());

    let errors: [IntPtr; 3] = [ib::isc_arg_sql_state as IntPtr, ib::isc_primary_key_notnull as IntPtr, ib::isc_arg_end as IntPtr];
    s.set_errors2(1, errors.as_ptr());
    println!("{}", Error::from_sw(&sw).text());
    // let mut a = unsafe { slice_from_raw_parts(s.get_errors(), 3).as_ref().unwrap() };
    // for i in a
    // {
    //     println!("3: {}", i);
    // }
}

#[test]
fn example_select()
{
    let mut rc = 0;
    std::env::set_var("ISC_USER", "lck");
    std::env::set_var("ISC_PASSWORD", "1");
    let master = Master::get();
    let st = master.get_status();
    let prov = master.get_dispatcher();
    let utl = master.get_util_interface();
    let status = StatusWrapper::new(&st);
    let att = prov.attach_database(&status, std::ffi::CString::new("/home/lck/.fbdb/test.fdb").unwrap().as_ptr(), 0, nullptr()).unwrap();

    let tpb = utl.get_xpb_builder(&status, XpbBuilder::TPB, std::ptr::null(), 0).unwrap();
    tpb.insert_tag(&status, ib::isc_tpb_read_committed as u8);
    tpb.insert_tag(&status, ib::isc_tpb_no_rec_version as u8);
    tpb.insert_tag(&status, ib::isc_tpb_wait as u8);
    tpb.insert_tag(&status, ib::isc_tpb_read as u8);
    let tra = att.start_transaction(&status, tpb.get_buffer_length(&status).unwrap(), tpb.get_buffer(&status).unwrap()).unwrap();

    // prepare statement
    let stmt = att.prepare(&status, &tra, 0, std::ffi::CString::new("select * from tet").unwrap().as_ptr(), ib::SQL_DIALECT_V6, Statement::PREPARE_PREFETCH_METADATA);
    if let Err(e) = stmt
    {

        println!("SQLCODE: {}", unsafe { ib::isc_sqlcode(st.get_errors()) });
        //unsafe { ib::isc_print_status(st.get_errors()) };
    }
    // if status.has_data() as u8 == 1
    // {
    //     println!("ERR: {}", Error::from_sw(&status).text());
    //     exit(0);
    // }
    // else { exit(0); }
    // get list of columns
    // let meta = stmt.get_output_metadata(&status);
    // let builder = meta.get_builder(&status);
    // let cols = meta.get_count(&status);
    // println!("Columns: {}", cols);
    // for i in 0..cols
    // {
    //     unsafe
    //     {
    //         println!("{}", CStr::from_ptr(meta.get_field(&status, i)).to_str().unwrap());
    //     }
    // }
}