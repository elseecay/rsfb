use crate::fbapi::*;
use std::str;
use std::ffi::CStr;
use std::process::exit;

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
    let att = prov.create_database(&status, std::ffi::CString::new("8.fdb").unwrap().as_ptr(), dpb.get_buffer_length(&status), dpb.get_buffer(&status));
    unsafe
    {
        if st.get_state() & Status::STATE_ERRORS != 0
        {
            let mut buffer: [Char; 1000] = [0; 1000];
            utl.format_status(buffer.as_mut_ptr(), 1000, &st);
            println!("Status formatted");
            println!("ERROR: {}", CStr::from_ptr(buffer.as_ptr()).to_str().expect("xxx"));
            exit(0);
        }
    }
    println!("Database created\n");
    att.detach(&status);
}

#[test]
fn example_select()
{

}