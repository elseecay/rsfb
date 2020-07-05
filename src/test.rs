use crate::fbapi as fb;
use crate::fbapi::provider_create_database;

#[test]
fn it_works()
{
    unsafe
    {
        let x = fb::fb_get_master_interface();
        println!("Called! it_works");
    }
}

#[test]
fn example_create_database()
{
    let m = fb::Master::new();
    let mut rc = 0;
    std::env::set_var("ISC_USER", "lck");
    std::env::set_var("ISC_PASSWORD", "666");
    unsafe
    {

        let master = fb::fb_get_master_interface();


        // set default password if none specified in environment


        // Declare pointers to required interfaces
        // IStatus is used to return wide error description to user
        // IProvider is needed to start to work with database (or service)
        // Status vector, main dispatcher and utility interfaces are returned by IMaster functions
        // No error return may happen - these functions always succeed
        let st = fb::master_get_status(master);
        let prov = fb::master_get_dispatcher(master);
        let utl = fb::master_get_util_interface(master);

        // IAttachment and ITransaction contain methods to work with database attachment
        // and transactions
        let att: fb::AttachmentPtr;
        let tra: fb::TransactionPtr;

        // IXpbBuilder is used to access various parameters blocks used in API
        let dpb: fb::XpbBuilderPtr;

        //Status wrapper - will be used later in all calls where status interface is needed
        //With ThrowStatusWrapper passed as status interface FbException will be thrown on error

        let status: fb::StatusWrapperPtr = fb::status_wrapper_new(st);

        // create DPB - use non-default page size 4Kb
        dpb = fb::util_get_xpb_builder(utl, status, 1u32, std::ptr::null::<u8>(), 0);
        fb::xpb_builder_insert_int(dpb, status, 4, 4 * 1024);

        // create empty database
        att = fb::provider_create_database(prov, status, std::ffi::CString::new("rustdb.fdb").expect("123").as_ptr(),
                                           fb::xpb_builder_get_buffer_length(dpb, status), fb::xpb_builder_get_buffer(dpb, status));
        println!("Database rustdb.fdb created\n");

        // // detach from database
        // att->detach(&status);
        // att = NULL;
        //
        // // attach it once again
        // att = prov->attachDatabase(&status, "fbtests.fdb", 0, NULL);
        // printf("Re-attached database fbtests.fdb\n");
        //
        // // start transaction
        // tra = att->startTransaction(&status, 0, NULL);
        //
        // // create table
        // att->execute(&status, tra, 0, "create table dates_table (d1 date)", SAMPLES_DIALECT,
        //              NULL, NULL, NULL, NULL);	// Input parameters and output data not used
        //
        // // commit transaction retaining
        // tra->commitRetaining(&status);
        // printf("Table dates_table created\n");
        //
        // // insert a record into dates_table
        // att->execute(&status, tra, 0, "insert into dates_table values (CURRENT_DATE)", SAMPLES_DIALECT,
        //              NULL, NULL, NULL, NULL);	// Input parameters and output data not used
        //
        // // commit transaction (will close interface)
        // tra->commit(&status);
        // tra = NULL;
        //
        // printf("Record inserted into dates_table\n");

        // detach from database (will close interface)
        fb::attachment_detach(att, status);
        fb::disposable_dispose(dpb);
        fb::disposable_dispose(st);
        fb::reference_counted_release(prov);
    }
    println!("Called! example");
}