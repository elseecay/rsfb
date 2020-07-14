use crate::detail::util::share::*;

use crate::*;

use crate::component::pbuilder::*;
use crate::component::pbuilder as pb;



#[test]
fn example_create_database()
{
    let mut b = pb::CreateDatabase::new().unwrap();
    b.set_user("user");
    b.set_password("password");
    b.set_page_size(4096);
    Connection::create_database("666.fdb", b).unwrap();
}

#[test]
fn example_attach_database()
{
    let mut b = pb::Connect::new().unwrap();
    b.set_user("user");
    b.set_password("password");
    let con = Connection::connect("665.fdb", b).unwrap();
}

#[test]
fn example_select()
{
    // let mut rc = 0;
    // std::env::set_var("ISC_USER", "lck");
    // std::env::set_var("ISC_PASSWORD", "1");
    // let master = Master::get();
    // let st = master.get_status();
    // let prov = master.get_dispatcher();
    // let utl = master.get_util_interface();
    // let status = StatusWrapper::new(&st);
    // let att = prov.attach_database(&status, std::ffi::CString::new("/home/lck/.fbdb/test.fdb").unwrap().as_ptr(), 0, nullptr()).unwrap();
    //
    // let tpb = utl.get_xpb_builder(&status, XpbBuilder::TPB, std::ptr::null(), 0).unwrap();
    // tpb.insert_tag(&status, ib::isc_tpb_read_committed as u8);
    // tpb.insert_tag(&status, ib::isc_tpb_no_rec_version as u8);
    // tpb.insert_tag(&status, ib::isc_tpb_wait as u8);
    // tpb.insert_tag(&status, ib::isc_tpb_read as u8);
    // let tra = att.start_transaction(&status, tpb.get_buffer_length(&status).unwrap(), tpb.get_buffer(&status).unwrap()).unwrap();
    //
    // // prepare statement
    // let stmt = att.prepare(&status, &tra, 0, std::ffi::CString::new("select * from tet").unwrap().as_ptr(), ib::SQL_DIALECT_V6, Statement::PREPARE_PREFETCH_METADATA);
    // if let Err(e) = stmt
    // {
    //
    //     println!("SQLCODE: {}", unsafe { ib::isc_sqlcode(st.get_errors()) });
    //     //unsafe { ib::isc_print_status(st.get_errors()) };
    // }
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