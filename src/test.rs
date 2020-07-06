use crate::fbapi::*;


#[test]
fn example_create_database()
{
    let mut rc = 0;
    std::env::set_var("ISC_USER", "lck");
    std::env::set_var("ISC_PASSWORD", "666");
    let master = Master::get();
    let st = master.get_status();
    let prov = master.get_dispatcher();
    let utl = master.get_util_interface();
    let status = StatusWrapper::new(&st);
    let dpb = utl.get_xpb_builder(&status, 1, std::ptr::null::<u8>(), 0);
    dpb.insert_int(&status, 4, 4 * 1024);
    let att = prov.create_database(&status, std::ffi::CString::new("rustdb2.fdb").expect("123").as_ptr(), dpb.get_buffer_length(&status), dpb.get_buffer(&status));
    println!("Database rustdb.fdb created\n");
    att.detach(&status);
    dpb.dispose();
    st.dispose();
    prov.release();
    println!("Called! example");
}