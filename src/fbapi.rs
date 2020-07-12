pub mod ibase;
pub use ibase as ib;
use crate::types::*;
use crate::{Error, Result};


trait CxxClass : Sized
{
    fn get_this(&self) -> VoidPtr;
    fn get_cthis(&self) -> VoidCPtr;
    fn is_destroyed(&self) -> bool;
    fn set_state_destroyed(&mut self);
}

macro_rules! declare_cxx_class
{
    ($classname: ident, $ptrname: ident, $cptrname: ident) =>
    {
        pub type $ptrname = VoidPtr;
        pub type $cptrname = VoidCPtr;
        pub struct $classname
        {
            this: VoidPtr
        }
        impl CxxClass for $classname
        {
            fn get_this(&self) -> VoidPtr
            {
                self.this
            }
            fn get_cthis(&self) -> VoidCPtr
            {
                self.this
            }
            fn is_destroyed(&self) -> bool
            {
                self.this == std::ptr::null_mut::<Void>()
            }
            fn set_state_destroyed(&mut self)
            {
                self.this = std::ptr::null_mut::<Void>();
            }
        }
        impl Drop for $classname
        {
            fn drop(&mut self)
            {
                if !self.is_destroyed()
                {
                    self.on_drop()
                }
            }
        }
    };
}

macro_rules! impl_as_def
{
    ($classname: ident, $($traitname: ident), +) =>
    {
        $(impl $traitname for $classname { })+
    }
}

declare_cxx_class!(Versioned, VersionedPtr, VersionedCPtr);
declare_cxx_class!(Disposable, DisposablePtr, DisposableCPtr);
declare_cxx_class!(ReferenceCounted, ReferenceCountedPtr, ReferenceCountedCPtr);
declare_cxx_class!(Master, MasterPtr, MasterCPtr);
declare_cxx_class!(Status, StatusPtr, StatusCPtr);
declare_cxx_class!(StatusWrapper, StatusWrapperPtr, StatusWrapperCPtr);
declare_cxx_class!(PluginBase, PluginBasePtr, PluginBaseCPtr);
declare_cxx_class!(Provider, ProviderPtr, ProviderCPtr);
declare_cxx_class!(Util, UtilPtr, UtilCPtr);
declare_cxx_class!(Attachment, AttachmentPtr, AttachmentCPtr);
declare_cxx_class!(XpbBuilder, XpbBuilderPtr, XpbBuilderCPtr);
declare_cxx_class!(Transaction, TransactionPtr, TransactionCPtr);
declare_cxx_class!(MessageMetadata, MessageMetadataPtr, MessageMetadataCPtr);
declare_cxx_class!(MetadataBuilder, MetadataBuilderPtr, MetadataBuilderCPtr);
declare_cxx_class!(Statement, StatementPtr, StatementCPtr);
declare_cxx_class!(Events, EventsPtr, EventsCPtr);
declare_cxx_class!(EventCallback, EventCallbackPtr, EventCallbackCPtr);
declare_cxx_class!(Request, RequestPtr, RequestCPtr);
declare_cxx_class!(ResultSet, ResultSetPtr, ResultSerCPtr);
declare_cxx_class!(PluginManager, PluginManagerPtr, PluginManagerCPtr);
declare_cxx_class!(TimerControl, TimerControlPtr, TimerControlCPtr);
declare_cxx_class!(Dtc, DtcPtr, DtcCPtr);
declare_cxx_class!(ConfigManager, ConfigManagerPtr, ConfigManagerCPtr);
declare_cxx_class!(Service, ServicePtr, ServiceCPtr);
declare_cxx_class!(CryptKeyCallback, CryptKeyCallbackPtr, CryptKeyCallbackCPtr);
declare_cxx_class!(VersionCallback, VersionCallbackPtr, VersionCallbackCPtr);
declare_cxx_class!(OffsetsCallback, OffsetsCallbackPtr, OffsetsCallbackCPtr);

#[link(name = "cfbapi")]
extern "C"
{
    // StatusWrapper
    pub fn status_wrapper_new(status: StatusPtr) -> StatusWrapperPtr;
    pub fn status_wrapper_delete(this: StatusWrapperPtr);
    pub fn status_wrapper_clear_exception(this: StatusWrapperPtr);
    pub fn status_wrapper_dispose(this: StatusWrapperPtr);
    pub fn status_wrapper_init(this: StatusWrapperPtr);
    pub fn status_wrapper_get_state(this: StatusWrapperCPtr) -> UInt;
    pub fn status_wrapper_set_errors2(this: StatusWrapperPtr, length: UInt, value: CPtr<IntPtr>);
    pub fn status_wrapper_set_warnings2(this: StatusWrapperPtr, length: UInt, value: CPtr<IntPtr>);
    pub fn status_wrapper_set_errors(this: StatusWrapperPtr, value: CPtr<IntPtr>);
    pub fn status_wrapper_set_warnings(this: StatusWrapperPtr, value: CPtr<IntPtr>);
    pub fn status_wrapper_get_errors(this: StatusWrapperCPtr) -> CPtr<IntPtr>;
    pub fn status_wrapper_get_warnings(this: StatusWrapperCPtr) -> CPtr<IntPtr>;
    pub fn status_wrapper_clone(this: StatusWrapperCPtr) -> StatusPtr;
    pub fn static_wrapper_is_dirty(this: StatusWrapperCPtr) -> FbBoolean;
    pub fn static_wrapper_has_data(this: StatusWrapperCPtr) -> FbBoolean;
    pub fn static_wrapper_is_empty(this: StatusWrapperCPtr) -> FbBoolean;

    // IDisposable
    pub fn disposable_dispose(this: DisposablePtr);

    // IReferenceCounted
    pub fn reference_counted_add_ref(this: ReferenceCountedPtr);
    pub fn reference_counted_release(this: ReferenceCountedPtr) -> Int;

    // IMaster
    pub fn master_get_status(this: MasterPtr) -> StatusPtr;
    pub fn master_get_dispatcher(this: MasterPtr) -> ProviderPtr;
    pub fn master_get_plugin_manager(this: MasterPtr) -> PluginManagerPtr;
    pub fn master_get_timer_control(this: MasterPtr) -> TimerControlPtr;
    pub fn master_get_dtc(this: MasterPtr) -> DtcPtr;
    pub fn master_register_attachment(this: MasterPtr, provider: ProviderPtr, attachment: AttachmentPtr) -> AttachmentPtr;
    pub fn master_register_transaction(this: MasterPtr, attachment: AttachmentPtr, transaction: TransactionPtr) -> TransactionPtr;
    pub fn master_get_metadata_builder(this: MasterPtr, status: StatusWrapperPtr, field_count: UInt) -> MetadataBuilderPtr;
    pub fn master_server_mode(this: MasterPtr, mode: Int) -> Int;
    pub fn master_get_util_interface(this: MasterPtr) -> UtilPtr;
    pub fn master_get_config_manager(this: MasterPtr) -> ConfigManagerPtr;
    pub fn master_get_process_exiting(this: MasterPtr) -> FbBoolean;

    // IStatus
    pub fn status_init(this: StatusPtr);
    pub fn status_get_state(this: StatusCPtr) -> UInt;
    pub fn status_set_errors2(this: StatusPtr, length: UInt, value: CPtr<IntPtr>);
    pub fn status_set_warnings2(this: StatusPtr, length: UInt, value: CPtr<IntPtr>);
    pub fn status_set_errors(this: StatusPtr, value: CPtr<IntPtr>);
    pub fn status_set_warnings(this: StatusPtr, value: CPtr<IntPtr>);
    pub fn status_get_errors(this: StatusCPtr) -> CPtr<IntPtr>;
    pub fn status_get_warnings(this: StatusCPtr) -> CPtr<IntPtr>;
    pub fn status_clone(this: StatusCPtr) -> StatusPtr;

    // IProvider
    pub fn provider_shutdown(this: ProviderPtr, status: StatusWrapperPtr, timeout: UInt, reason: Int);
    pub fn provider_attach_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> AttachmentPtr;
    pub fn provider_create_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> AttachmentPtr;
    pub fn provider_attach_service_manager(this: ProviderPtr, status: StatusWrapperPtr, service: CPtr<Char>, spb_length: UInt, spb: CPtr<UChar>) -> ServicePtr;
    pub fn provider_set_db_crypt_callback(this: ProviderPtr, status: StatusWrapperPtr, crypt_callback: CryptKeyCallbackPtr);

    // IUtil
    pub fn util_get_fb_version(this: UtilPtr, status: StatusWrapperPtr, att: AttachmentPtr, callback: VersionCallbackPtr);
    pub fn util_load_blob(this: UtilPtr, status: StatusWrapperPtr, blob_id: Ptr<IscQuad>, att: AttachmentPtr, tra: TransactionPtr, file: CPtr<Char>, txt: FbBoolean);
    pub fn util_dump_blob(this: UtilPtr, status: StatusWrapperPtr, blob_id: Ptr<IscQuad>, att: AttachmentPtr, tra: TransactionPtr, file: CPtr<Char>, txt: FbBoolean);
    pub fn util_get_perf_counters(this: UtilPtr, status: StatusWrapperPtr, att: AttachmentPtr, counters_set: CPtr<Char>, counters: Ptr<IscInt64>);
    pub fn util_execute_create_database(this: UtilPtr, status: StatusWrapperPtr, stmt_length: UInt, create_db_statement: CPtr<Char>, dialect: UInt, stmt_is_create_db: Ptr<FbBoolean>) -> AttachmentPtr;
    pub fn util_decode_date(this: UtilPtr, date: IscDate, year: Ptr<UInt>, month: Ptr<UInt>, day: Ptr<UInt>);
    pub fn util_decode_time(this: UtilPtr, time: IscTime, hours: Ptr<UInt>, minutes: Ptr<UInt>, seconds: Ptr<UInt>, fractions: Ptr<UInt>);
    pub fn util_encode_date(this: UtilPtr, year: UInt, month: UInt, day: UInt) -> IscDate;
    pub fn util_encode_time(this: UtilPtr, hours: UInt, minutes: UInt, seconds: UInt, fractions: UInt) -> IscTime;
    pub fn util_format_status(this: UtilPtr, buffer: Ptr<Char>, buffer_size: UInt, status: StatusPtr) -> UInt;
    pub fn util_get_client_version(this: UtilPtr) -> UInt;
    pub fn util_get_xpb_builder(this: UtilPtr, status: StatusWrapperPtr, kind: UInt, buf: CPtr<UChar>, len: UInt) -> XpbBuilderPtr;
    pub fn util_set_offsets(this: UtilPtr, status: StatusWrapperPtr, metadata: MessageMetadataPtr, callback: OffsetsCallbackPtr) -> UInt;

    // IXpbBuilder
    pub fn xpb_builder_clear(this: XpbBuilderPtr, status: StatusWrapperPtr);
    pub fn xpb_builder_remove_current(this: XpbBuilderPtr, status: StatusWrapperPtr);
    pub fn xpb_builder_insert_int(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, value: Int);
    pub fn xpb_builder_insert_big_int(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, value: IscInt64);
    pub fn xpb_builder_insert_bytes(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, bytes: CPtr<Void>, length: UInt);
    pub fn xpb_builder_insert_string(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, str: CPtr<Char>);
    pub fn xpb_builder_insert_tag(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar);
    pub fn xpb_builder_is_eof(this: XpbBuilderPtr, status: StatusWrapperPtr) -> FbBoolean;
    pub fn xpb_builder_move_next(this: XpbBuilderPtr, status: StatusWrapperPtr);
    pub fn xpb_builder_rewind(this: XpbBuilderPtr, status: StatusWrapperPtr);
    pub fn xpb_builder_find_first(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar) -> FbBoolean;
    pub fn xpb_builder_find_next(this: XpbBuilderPtr, status: StatusWrapperPtr) -> FbBoolean;
    pub fn xpb_builder_get_tag(this: XpbBuilderPtr, status: StatusWrapperPtr) -> UChar;
    pub fn xpb_builder_get_length(this: XpbBuilderPtr, status: StatusWrapperPtr) -> UInt;
    pub fn xpb_builder_get_int(this: XpbBuilderPtr, status: StatusWrapperPtr) -> Int;
    pub fn xpb_builder_get_big_int(this: XpbBuilderPtr, status: StatusWrapperPtr) -> IscInt64;
    pub fn xpb_builder_get_string(this: XpbBuilderPtr, status: StatusWrapperPtr) -> CPtr<Char>;
    pub fn xpb_builder_get_bytes(this: XpbBuilderPtr, status: StatusWrapperPtr) -> CPtr<UChar>;
    pub fn xpb_builder_get_buffer_length(this: XpbBuilderPtr, status: StatusWrapperPtr) -> UInt;
    pub fn xpb_builder_get_buffer(this: XpbBuilderPtr, status: StatusWrapperPtr) -> CPtr<UChar>;

    // IAttachment
    pub fn attachment_detach(this: AttachmentPtr, status: StatusWrapperPtr); // object destroyed
    pub fn attachment_prepare(this: AttachmentPtr, status: StatusWrapperPtr, tra: TransactionPtr, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, flags: UInt) -> StatementPtr;
    pub fn attachment_start_transaction(this: AttachmentPtr, status: StatusWrapperPtr, tpb_length: UInt, tpb: CPtr<UChar>) -> TransactionPtr;
    pub fn attachment_execute(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, in_metadata: MessageMetadataPtr, in_buffer: VoidPtr, out_metadata: MessageMetadataPtr, out_buffer: VoidPtr) -> TransactionPtr;
    pub fn attachment_get_info(this: AttachmentPtr, status: StatusWrapperPtr, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>);
    pub fn attachment_reconnect_transaction(this: AttachmentPtr, status: StatusWrapperPtr, length: UInt, id: CPtr<UChar>) -> TransactionPtr;
    pub fn attachment_compile_request(this: AttachmentPtr, status: StatusWrapperPtr, blr_length: UInt, blr: CPtr<UChar>) -> RequestPtr;
    pub fn attachment_transact_request(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, blr_length: UInt, blr: CPtr<UChar>, in_msg_length: UInt, in_msg: CPtr<UChar>, out_msg_length: UInt, out_msg: Ptr<UChar>);
    pub fn attachment_execute_dyn(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, length: UInt, dn: CPtr<UChar>);
    pub fn attachment_open_cursor(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, in_metadata: MessageMetadataPtr, in_buffer: Ptr<Void>, out_metadata: MessageMetadataPtr, cursor_name: CPtr<Char>, cursor_flags: UInt) -> ResultSetPtr;
    pub fn attachment_que_events(this: AttachmentPtr, status: StatusWrapperPtr, callback: EventCallbackPtr, length: UInt, events: CPtr<UChar>) -> EventsPtr;
    pub fn attachment_cancel_operation(this: AttachmentPtr, status: StatusWrapperPtr, option: Int);
    pub fn attachment_ping(this: AttachmentPtr, status: StatusWrapperPtr);
    pub fn attachment_drop_database(this: AttachmentPtr, status: StatusWrapperPtr);

    // ITransaction
    pub fn transaction_get_info(this: TransactionPtr, status: StatusWrapperPtr, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>);
    pub fn transaction_prepare(this: TransactionPtr, status: StatusWrapperPtr, msg_length: UInt, message: CPtr<UChar>);
    pub fn transaction_commit(this: TransactionPtr, status: StatusWrapperPtr); // object destroyed
    pub fn transaction_commit_retaining(this: TransactionPtr, status: StatusWrapperPtr);
    pub fn transaction_rollback(this: TransactionPtr, status: StatusWrapperPtr);
    pub fn transaction_rollback_retaining(this: TransactionPtr, status: StatusWrapperPtr);
    pub fn transaction_disconnect(this: TransactionPtr, status: StatusWrapperPtr);
    pub fn transaction_join(this: TransactionPtr, status: StatusWrapperPtr, transaction: TransactionPtr) -> TransactionPtr;
    pub fn transaction_validate(this: TransactionPtr, status: StatusWrapperPtr, attachment: AttachmentPtr) -> TransactionPtr;
    pub fn transaction_enter_dtc(this: TransactionPtr, status: StatusWrapperPtr) -> TransactionPtr;

    // IStatement
    pub fn statement_get_info(this: StatementPtr, status: StatusWrapperPtr, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>);
    pub fn statement_get_type(this: StatementPtr, status: StatusWrapperPtr) -> UInt;
    pub fn statement_get_plan(this: StatementPtr, status: StatusWrapperPtr, detailed: FbBoolean) -> CPtr<Char>;
    pub fn statement_get_affected_records(this: StatementPtr, status: StatusWrapperPtr) -> IscUInt64;
    pub fn statement_get_input_metadata(this: StatementPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;
    pub fn statement_get_output_metadata(this: StatementPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;
    pub fn statement_execute(this: StatementPtr, status: StatusWrapperPtr, transaction: TransactionPtr, in_metadata: MessageMetadataPtr, in_buffer: Ptr<Void>, out_metadata: MessageMetadataPtr, out_buffer: Ptr<Void>) -> TransactionPtr;
    pub fn statement_open_cursor(this: StatementPtr, status: StatusWrapperPtr, transaction: TransactionPtr, in_metadata: MessageMetadataPtr, in_buffer: Ptr<Void>, out_metadata: MessageMetadataPtr, flags: UInt) -> ResultSetPtr;
    pub fn statement_set_cursor_name(this: StatementPtr, status: StatusWrapperPtr, name: CPtr<Char>);
    pub fn statement_free(this: StatementPtr, status: StatusWrapperPtr);
    pub fn statement_get_flags(this: StatementPtr, status: StatusWrapperPtr) -> UInt;


    // IMessageMetadata
    pub fn message_metadata_get_count(this: MessageMetadataPtr, status: StatusWrapperPtr) -> UInt;
    pub fn message_metadata_get_field(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> CPtr<Char>;
    pub fn message_metadata_get_relation(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> CPtr<Char>;
    pub fn message_metadata_get_owner(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> CPtr<Char>;
    pub fn message_metadata_get_alias(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> CPtr<Char>;
    pub fn message_metadata_get_type(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> UInt;
    pub fn message_metadata_is_nullable(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> FbBoolean;
    pub fn message_metadata_get_sub_type(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> Int;
    pub fn message_metadata_get_length(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> UInt;
    pub fn message_metadata_get_scale(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> Int;
    pub fn message_metadata_get_char_set(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> UInt;
    pub fn message_metadata_get_offset(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> UInt;
    pub fn message_metadata_get_null_offset(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> UInt;
    pub fn message_metadata_get_builder(this: MessageMetadataPtr, status: StatusWrapperPtr) -> MetadataBuilderPtr;
    pub fn message_metadata_get_message_length(this: MessageMetadataPtr, status: StatusWrapperPtr) -> UInt;

    // IMetadataBuilder
    pub fn metadata_builder_set_type(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: UInt, typ: UInt);
    pub fn metadata_builder_set_sub_type(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: UInt, sub_type: Int);
    pub fn metadata_builder_set_length(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: UInt, length: UInt);
    pub fn metadata_builder_set_char_set(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: UInt, char_set: UInt);
    pub fn metadata_builder_set_scale(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: UInt, scale: Int);
    pub fn metadata_builder_truncate(this: MetadataBuilderPtr, status: StatusWrapperPtr, count: UInt);
    pub fn metadata_builder_move_name_to_index(this: MetadataBuilderPtr, status: StatusWrapperPtr, name: CPtr<Char>, index: UInt);
    pub fn metadata_builder_remove(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: UInt);
    pub fn metadata_builder_add_field(this: MetadataBuilderPtr, status: StatusWrapperPtr) -> UInt;
    pub fn metadata_builder_get_metadata(this: MetadataBuilderPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;

    // IResultSet
    pub fn result_set_fetch_next(this: ResultSetPtr, status: StatusWrapperPtr, message: Ptr<Void>) -> Int;
    pub fn result_set_fetch_prior(this: ResultSetPtr, status: StatusWrapperPtr, message: Ptr<Void>) -> Int;
    pub fn result_set_fetch_first(this: ResultSetPtr, status: StatusWrapperPtr, message: Ptr<Void>) -> Int;
    pub fn result_set_fetch_last(this: ResultSetPtr, status: StatusWrapperPtr, message: Ptr<Void>) -> Int;
    pub fn result_set_fetch_absolute(this: ResultSetPtr, status: StatusWrapperPtr, position: Int, message: Ptr<Void>) -> Int;
    pub fn result_set_fetch_relative(this: ResultSetPtr, status: StatusWrapperPtr, offset: Int, message: Ptr<Void>) -> Int;
    pub fn result_set_is_eof(this: ResultSetPtr, status: StatusWrapperPtr) -> FbBoolean;
    pub fn result_set_is_bof(this: ResultSetPtr, status: StatusWrapperPtr) -> FbBoolean;
    pub fn result_set_get_metadata(this: ResultSetPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;
    pub fn result_set_close(this: ResultSetPtr, status: StatusWrapperPtr);
    pub fn result_set_set_delayed_output_format(this: ResultSetPtr, status: StatusWrapperPtr, format: MessageMetadataPtr);
}

#[link(name = "fbclient")]
extern "C"
{
    pub fn fb_get_master_interface() -> MasterPtr;
}

pub trait IVersioned : CxxClass
{
    fn on_drop(&mut self){ }
}

impl_as_def!(Versioned, IVersioned);

pub trait IDisposable : CxxClass
{
    fn dispose(&mut self)
    {
        unsafe { disposable_dispose(self.get_this() as DisposablePtr); }
    }
    fn on_drop(&mut self)
    {
        self.dispose();
    }
}

impl_as_def!(Disposable, IDisposable);

pub trait IReferenceCounted : CxxClass
{
    fn add_ref(&self)
    {
        unsafe { reference_counted_add_ref(self.get_this() as ReferenceCountedPtr); }
    }
    fn release(&mut self)
    {
        unsafe { reference_counted_release(self.get_this() as ReferenceCountedPtr); }
    }
    fn on_drop(&mut self)
    {
        self.release();
    }
}

impl_as_def!(ReferenceCounted, IReferenceCounted);

pub trait IDeletable : CxxClass // custom destruction operation
{
    fn delete(&mut self);
    fn on_drop(&mut self)
    {
        self.delete();
    }
}

pub trait IStatus : IDisposable
{
    const STATE_WARNINGS: UInt = 1;
    const STATE_ERRORS: UInt = 2;
    const RESULT_ERROR: Int = -1;
    const RESULT_OK: Int = 0;
    const RESULT_NO_DATA: Int = 1;
    const RESULT_SEGMENT: Int = 2;

    fn init(&self)
    {
        unsafe { return status_init(self.get_this()); }
    }
    fn get_state(&self) -> UInt
    {
        unsafe { return status_get_state(self.get_cthis()); }
    }
    fn set_errors2(&self, length: UInt, value: CPtr<IntPtr>)
    {
        unsafe { return status_set_errors2(self.get_this(), length, value); }
    }
    fn set_warnings2(&self, length: UInt, value: CPtr<IntPtr>)
    {
        unsafe { return status_set_warnings2(self.get_this(), length, value); }
    }
    fn set_errors(&self, value: CPtr<IntPtr>)
    {
        unsafe { return status_set_errors(self.get_this(), value); }
    }
    fn set_warnings(&self, value: CPtr<IntPtr>)
    {
        unsafe { return status_set_warnings(self.get_this(), value); }
    }
    fn get_errors(&self) -> CPtr<IntPtr>
    {
        unsafe { return status_get_errors(self.get_cthis()); }
    }
    fn get_warnings(&self) -> CPtr<IntPtr>
    {
        unsafe { return status_get_warnings(self.get_cthis()); }
    }
    fn clone(&self) -> Status
    {
        unsafe { return Status{ this: status_clone(self.get_cthis()) }; }
    }
}

impl_as_def!(Status, IDisposable, IStatus);

// TODO: rework
pub trait IStatusWrapper : IDeletable
{
    fn new(status: &Status) -> StatusWrapper
    {
        unsafe { return StatusWrapper{ this: status_wrapper_new(status.get_this()) }; }
    }
    fn clear_exception(&self)
    {
        unsafe { return status_wrapper_clear_exception(self.get_this()); }
    }
    fn dispose(&self) // disposes internal IStatus
    {
        unsafe { return status_wrapper_dispose(self.get_this()); }
    }
    fn init(&self)
    {
        unsafe { return status_wrapper_init(self.get_this()); }
    }
    fn get_state(&self) -> UInt
    {
        unsafe { return status_wrapper_get_state(self.get_cthis()); }
    }
    fn set_errors2(&self, length: UInt, value: CPtr<IntPtr>)
    {
        unsafe { return status_wrapper_set_errors2(self.get_this(), length, value); }
    }
    fn set_warnings2(&self, length: UInt, value: CPtr<IntPtr>)
    {
        unsafe { return status_wrapper_set_warnings2(self.get_this(), length, value); }
    }
    fn set_errors(&self, value: CPtr<IntPtr>)
    {
        unsafe { return status_wrapper_set_errors(self.get_this(), value); }
    }
    fn set_warnings(&self, value: CPtr<IntPtr>)
    {
        unsafe { return status_wrapper_set_warnings(self.get_this(), value); }
    }
    fn get_errors(&self) -> CPtr<IntPtr>
    {
        unsafe { return status_wrapper_get_errors(self.get_cthis()); }
    }
    fn get_warnings(&self) -> CPtr<IntPtr>
    {
        unsafe { return status_wrapper_get_warnings(self.get_cthis()); }
    }
    fn clone(&self) -> Status
    {
        unsafe { return Status{ this: status_wrapper_clone(self.get_cthis()) }; }
    }
    fn is_dirty(&self) -> FbBoolean
    {
        unsafe { return static_wrapper_is_dirty(self.get_this()); }
    }
    fn has_data(&self) -> FbBoolean
    {
        unsafe { return static_wrapper_has_data(self.get_this()); }
    }
    fn is_empty(&self) -> FbBoolean
    {
        unsafe { return static_wrapper_is_empty(self.get_this()); }
    }
}

impl IDeletable for StatusWrapper
{
    fn delete(&mut self)
    {
        unsafe { status_wrapper_delete(self.get_this()); }
    }
}

impl_as_def!(StatusWrapper, IStatusWrapper);

pub trait IMaster : IVersioned
{
    fn get() -> Master
    {
        unsafe { return Master{ this: fb_get_master_interface() }; }
    }
    fn get_status(&self) -> Status
    {
        unsafe { return Status{ this: master_get_status(self.get_this()) }; }
    }
    fn get_dispatcher(&self) -> Provider
    {
        unsafe { return Provider{ this: master_get_dispatcher(self.get_this()) }; }
    }
    fn get_plugin_manager(&self) -> PluginManager
    {
        unsafe { return PluginManager{ this: master_get_plugin_manager(self.get_this()) }; }
    }
    fn get_timer_control(&self) -> TimerControl
    {
        unsafe { return TimerControl{ this: master_get_timer_control(self.get_this()) }; }
    }
    fn get_dtc(&self) -> Dtc
    {
        unsafe { return Dtc{ this: master_get_dtc(self.get_this()) }; }
    }
    fn register_attachment(&self, provider: &Provider, attachment: &Attachment) -> Attachment
    {
        unsafe { return Attachment{ this: master_register_attachment(self.get_this(), provider.this, attachment.this) }; }
    }
    fn register_transaction(&self, attachment: &Attachment, transaction: &Transaction) -> Transaction
    {
        unsafe { return Transaction{ this: master_register_transaction(self.get_this(), attachment.this, transaction.this) }; }
    }
    fn get_metadata_builder(&self, status: &StatusWrapper, field_count: UInt) -> Result<MetadataBuilder>
    {
        let result = unsafe { MetadataBuilder{ this: master_get_metadata_builder(self.get_this(), status.this, field_count) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn server_mode(&self, mode: Int) -> Int
    {
        unsafe { return master_server_mode(self.get_this(), mode); }
    }
    fn get_util_interface(&self) -> Util
    {
        unsafe { return Util{ this: master_get_util_interface(self.get_this()) }; }
    }
    fn get_config_manager(&self) -> ConfigManager
    {
        unsafe { return ConfigManager{ this: master_get_config_manager(self.get_this()) }; }
    }
    fn get_process_exiting(&self) -> FbBoolean
    {
        unsafe { return master_get_process_exiting(self.get_this()); }
    }
}

impl_as_def!(Master, IVersioned, IMaster);

pub trait IUtil : IVersioned
{
    fn get_fb_version(&self, status: &StatusWrapper, att: &Attachment, callback: &VersionCallback) -> Result<()>
    {
        let result = unsafe { util_get_fb_version(self.get_this(), status.this, att.this, callback.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn load_blob(&self, status: &StatusWrapper, blob_id: Ptr<IscQuad>, att: &Attachment, tra: &Transaction, file: CPtr<Char>, txt: FbBoolean) -> Result<()>
    {
        let result = unsafe { util_load_blob(self.get_this(), status.this, blob_id, att.this, tra.this, file, txt) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn dump_blob(&self, status: &StatusWrapper, blob_id: Ptr<IscQuad>, att: &Attachment, tra: &Transaction, file: CPtr<Char>, txt: FbBoolean) -> Result<()>
    {
        let result = unsafe { util_dump_blob(self.get_this(), status.this, blob_id, att.this, tra.this, file, txt) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_perf_counters(&self, status: &StatusWrapper, att: &Attachment, counters_set: CPtr<Char>, counters: Ptr<IscInt64>) -> Result<()>
    {
        let result = unsafe { util_get_perf_counters(self.get_this(), status.this, att.this, counters_set, counters) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn execute_create_database(&self, status: &StatusWrapper, stmt_length: UInt, create_db_statement: CPtr<Char>, dialect: UInt, stmt_is_create_db: Ptr<FbBoolean>) -> Result<Attachment>
    {
        let result = unsafe { Attachment{ this: util_execute_create_database(self.get_this(), status.this, stmt_length, create_db_statement, dialect, stmt_is_create_db) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn decode_date(&self, date: IscDate, year: Ptr<UInt>, month: Ptr<UInt>, day: Ptr<UInt>)
    {
        unsafe { return util_decode_date(self.get_this(), date, year, month, day); }
    }
    fn decode_time(&self, time: IscTime, hours: Ptr<UInt>, minutes: Ptr<UInt>, seconds: Ptr<UInt>, fractions: Ptr<UInt>)
    {
        unsafe { return util_decode_time(self.get_this(), time, hours, minutes, seconds, fractions); }
    }
    fn encode_date(&self, year: UInt, month: UInt, day: UInt) -> IscDate
    {
        unsafe { return util_encode_date(self.get_this(), year, month, day); }
    }
    fn encode_time(&self, hours: UInt, minutes: UInt, seconds: UInt, fractions: UInt) -> IscTime
    {
        unsafe { return util_encode_time(self.get_this(), hours, minutes, seconds, fractions); }
    }
    fn format_status(&self, buffer: Ptr<Char>, buffer_size: UInt, status: &Status) -> UInt
    {
        unsafe { return util_format_status(self.get_this(), buffer, buffer_size, status.this); }
    }
    fn get_client_version(&self) -> UInt
    {
        unsafe { return util_get_client_version(self.get_this()); }
    }
    fn get_xpb_builder(&self, status: &StatusWrapper, kind: UInt, buf: CPtr<UChar>, len: UInt) -> Result<XpbBuilder>
    {
        let result = unsafe { XpbBuilder{ this: util_get_xpb_builder(self.get_this(), status.this, kind, buf, len) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_offsets(&self, status: &StatusWrapper, metadata: &MessageMetadata, callback: &OffsetsCallback) -> Result<UInt>
    {
        let result = unsafe { util_set_offsets(self.get_this(), status.this, metadata.this, callback.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(Util, IVersioned, IUtil);

pub trait IPluginBase : IReferenceCounted
{

}

impl_as_def!(PluginBase, IReferenceCounted, IPluginBase);

pub trait IProvider : IPluginBase
{
    fn shutdown(&self, status: &StatusWrapper, timeout: UInt, reason: Int) -> Result<()>
    {
        let result = unsafe { provider_shutdown(self.get_this(), status.this, timeout, reason) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn attach_database(&self, status: &StatusWrapper, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> Result<Attachment>
    {
        let result = unsafe { Attachment{ this: provider_attach_database(self.get_this(), status.this, file_name, dpb_length, dpb) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn create_database(&self, status: &StatusWrapper, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> Result<Attachment>
    {
        let result = unsafe { Attachment{ this: provider_create_database(self.get_this(), status.this, file_name, dpb_length, dpb) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn attach_service_manager(&self, status: &StatusWrapper, service: CPtr<Char>, spb_length: UInt, spb: CPtr<UChar>) -> Result<Service>
    {
        let result = unsafe { Service{ this: provider_attach_service_manager(self.get_this(), status.this, service, spb_length, spb) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_db_crypt_callback(&self, status: &StatusWrapper, crypt_callback: &CryptKeyCallback) -> Result<()>
    {
        let result = unsafe { provider_set_db_crypt_callback(self.get_this(), status.this, crypt_callback.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(Provider, IReferenceCounted, IPluginBase, IProvider);

pub trait IXpbBuilder : IDisposable
{
    const DPB: UInt = 1;
    const SPB_ATTACH: UInt = 2;
    const SPB_START: UInt = 3;
    const TPB: UInt = 4;

    fn clear(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { xpb_builder_clear(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn remove_current(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { xpb_builder_remove_current(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn insert_int(&self, status: &StatusWrapper, tag: UChar, value: Int) -> Result<()>
    {
        let result = unsafe { xpb_builder_insert_int(self.get_this(), status.this, tag, value) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn insert_big_int(&self, status: &StatusWrapper, tag: UChar, value: IscInt64) -> Result<()>
    {
        let result = unsafe { xpb_builder_insert_big_int(self.get_this(), status.this, tag, value) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn insert_bytes(&self, status: &StatusWrapper, tag: UChar, bytes: CPtr<Void>, length: UInt) -> Result<()>
    {
        let result = unsafe { xpb_builder_insert_bytes(self.get_this(), status.this, tag, bytes, length) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn insert_string(&self, status: &StatusWrapper, tag: UChar, str: CPtr<Char>) -> Result<()>
    {
        let result = unsafe { xpb_builder_insert_string(self.get_this(), status.this, tag, str) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn insert_tag(&self, status: &StatusWrapper, tag: UChar) -> Result<()>
    {
        let result = unsafe { xpb_builder_insert_tag(self.get_this(), status.this, tag) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn is_eof(&self, status: &StatusWrapper) -> Result<FbBoolean>
    {
        let result = unsafe { xpb_builder_is_eof(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn move_next(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { xpb_builder_move_next(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn rewind(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { xpb_builder_rewind(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn find_first(&self, status: &StatusWrapper, tag: UChar) -> Result<FbBoolean>
    {
        let result = unsafe { xpb_builder_find_first(self.get_this(), status.this, tag) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn find_next(&self, status: &StatusWrapper) -> Result<FbBoolean>
    {
        let result = unsafe { xpb_builder_find_next(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_tag(&self, status: &StatusWrapper) -> Result<UChar>
    {
        let result = unsafe { xpb_builder_get_tag(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_length(&self, status: &StatusWrapper) -> Result<UInt>
    {
        let result = unsafe { xpb_builder_get_length(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_int(&self, status: &StatusWrapper) -> Result<Int>
    {
        let result = unsafe { xpb_builder_get_int(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_big_int(&self, status: &StatusWrapper) -> Result<IscInt64>
    {
        let result = unsafe { xpb_builder_get_big_int(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_string(&self, status: &StatusWrapper) -> Result<CPtr<Char>>
    {
        let result = unsafe { xpb_builder_get_string(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_bytes(&self, status: &StatusWrapper) -> Result<CPtr<UChar>>
    {
        let result = unsafe { xpb_builder_get_bytes(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_buffer_length(&self, status: &StatusWrapper) -> Result<UInt>
    {
        let result = unsafe { xpb_builder_get_buffer_length(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_buffer(&self, status: &StatusWrapper) -> Result<CPtr<UChar>>
    {
        let result = unsafe { xpb_builder_get_buffer(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(XpbBuilder, IDisposable, IXpbBuilder);

pub trait IAttachment : IReferenceCounted
{
    fn detach(self, status: &StatusWrapper) -> Result<()>
    {
        unsafe { attachment_detach(self.get_this(), status.this); }
        if status.has_data() != 0
        {
            return Err(Error::from_sw(&status));
        }
        return Ok(());
    }
    fn prepare(&self, status: &StatusWrapper, tra: &Transaction, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, flags: UInt) -> Result<Statement>
    {
        let result = unsafe { Statement{ this: attachment_prepare(self.get_this(), status.this, tra.this, stmt_length, sql_stmt, dialect, flags) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn start_transaction(&self, status: &StatusWrapper, tpb_length: UInt, tpb: CPtr<UChar>) -> Result<Transaction>
    {
        let result = unsafe { Transaction{ this: attachment_start_transaction(self.get_this(), status.this, tpb_length, tpb) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn execute(&self, status: &StatusWrapper, transaction: &Transaction, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, in_metadata: &MessageMetadata, in_buffer: VoidPtr, out_metadata: &MessageMetadata, out_buffer: VoidPtr) -> Result<Transaction>
    {
        let result = unsafe { Transaction{ this: attachment_execute(self.get_this(), status.this, transaction.this, stmt_length, sql_stmt, dialect, in_metadata.this, in_buffer, out_metadata.this, out_buffer) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_info(&self, status: &StatusWrapper, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>) -> Result<()>
    {
        let result = unsafe { attachment_get_info(self.get_this(), status.this, items_length, items, buffer_length, buffer) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn reconnect_transaction(&self, status: &StatusWrapper, length: UInt, id: CPtr<UChar>) -> Result<Transaction>
    {
        let result = unsafe { Transaction{ this: attachment_reconnect_transaction(self.get_this(), status.this, length, id) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn compile_request(&self, status: &StatusWrapper, blr_length: UInt, blr: CPtr<UChar>) -> Result<Request>
    {
        let result = unsafe { Request{ this: attachment_compile_request(self.get_this(), status.this, blr_length, blr) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn transact_request(&self, status: &StatusWrapper, transaction: &Transaction, blr_length: UInt, blr: CPtr<UChar>, in_msg_length: UInt, in_msg: CPtr<UChar>, out_msg_length: UInt, out_msg: Ptr<UChar>) -> Result<()>
    {
        let result = unsafe { attachment_transact_request(self.get_this(), status.this, transaction.this, blr_length, blr, in_msg_length, in_msg, out_msg_length, out_msg) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn execute_dyn(&self, status: &StatusWrapper, transaction: &Transaction, length: UInt, dn: CPtr<UChar>) -> Result<()>
    {
        let result = unsafe { attachment_execute_dyn(self.get_this(), status.this, transaction.this, length, dn) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn open_cursor(&self, status: &StatusWrapper, transaction: &Transaction, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, in_metadata: &MessageMetadata, in_buffer: Ptr<Void>, out_metadata: &MessageMetadata, cursor_name: CPtr<Char>, cursor_flags: UInt) -> Result<ResultSet>
    {
        let result = unsafe { ResultSet{ this: attachment_open_cursor(self.get_this(), status.this, transaction.this, stmt_length, sql_stmt, dialect, in_metadata.this, in_buffer, out_metadata.this, cursor_name, cursor_flags) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn que_events(&self, status: &StatusWrapper, callback: &EventCallback, length: UInt, events: CPtr<UChar>) -> Result<Events>
    {
        let result = unsafe { Events{ this: attachment_que_events(self.get_this(), status.this, callback.this, length, events) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn cancel_operation(&self, status: &StatusWrapper, option: Int) -> Result<()>
    {
        let result = unsafe { attachment_cancel_operation(self.get_this(), status.this, option) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn ping(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { attachment_ping(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn drop_database(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { attachment_drop_database(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(Attachment, IReferenceCounted, IAttachment);

pub trait ITransaction : IReferenceCounted
{
    fn get_info(&self, status: &StatusWrapper, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>) -> Result<()>
    {
        let result = unsafe { transaction_get_info(self.get_this(), status.this, items_length, items, buffer_length, buffer) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn prepare(&self, status: &StatusWrapper, msg_length: UInt, message: CPtr<UChar>) -> Result<()>
    {
        let result = unsafe { transaction_prepare(self.get_this(), status.this, msg_length, message) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn commit(self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { transaction_commit(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn commit_retaining(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { transaction_commit_retaining(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn rollback(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { transaction_rollback(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn rollback_retaining(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { transaction_rollback_retaining(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn disconnect(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { transaction_disconnect(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn join(&self, status: &StatusWrapper, transaction: &Transaction) -> Result<Transaction>
    {
        let result = unsafe { Transaction{ this: transaction_join(self.get_this(), status.this, transaction.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn validate(&self, status: &StatusWrapper, attachment: &Attachment) -> Result<Transaction>
    {
        let result = unsafe { Transaction{ this: transaction_validate(self.get_this(), status.this, attachment.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn enter_dtc(&self, status: &StatusWrapper) -> Result<Transaction>
    {
        let result = unsafe { Transaction{ this: transaction_enter_dtc(self.get_this(), status.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(Transaction, IReferenceCounted, ITransaction);

pub trait IStatement : IReferenceCounted
{
    const PREPARE_PREFETCH_NONE: UInt = 0;
    const PREPARE_PREFETCH_TYPE: UInt = 1;
    const PREPARE_PREFETCH_INPUT_PARAMETERS: UInt = 2;
    const PREPARE_PREFETCH_OUTPUT_PARAMETERS: UInt = 4;
    const PREPARE_PREFETCH_LEGACY_PLAN: UInt = 8;
    const PREPARE_PREFETCH_DETAILED_PLAN: UInt = 16;
    const PREPARE_PREFETCH_AFFECTED_RECORDS: UInt = 32;
    const PREPARE_PREFETCH_FLAGS: UInt = 64;
    const PREPARE_PREFETCH_METADATA: UInt = Self::PREPARE_PREFETCH_TYPE | Self::PREPARE_PREFETCH_FLAGS | Self::PREPARE_PREFETCH_INPUT_PARAMETERS | Self::PREPARE_PREFETCH_OUTPUT_PARAMETERS;
    const PREPARE_PREFETCH_ALL: UInt = Self::PREPARE_PREFETCH_METADATA | Self::PREPARE_PREFETCH_LEGACY_PLAN | Self::PREPARE_PREFETCH_DETAILED_PLAN | Self::PREPARE_PREFETCH_AFFECTED_RECORDS;
    const FLAG_HAS_CURSOR: UInt = 1;
    const FLAG_REPEAT_EXECUTE: UInt = 2;
    const CURSOR_TYPE_SCROLLABLE: UInt = 1;

    fn get_info(&self, status: &StatusWrapper, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>) -> Result<()>
    {
        let result = unsafe { statement_get_info(self.get_this(), status.this, items_length, items, buffer_length, buffer) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_type(&self, status: &StatusWrapper) -> Result<UInt>
    {
        let result = unsafe { statement_get_type(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_plan(&self, status: &StatusWrapper, detailed: FbBoolean) -> Result<CPtr<Char>>
    {
        let result = unsafe { statement_get_plan(self.get_this(), status.this, detailed) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_affected_records(&self, status: &StatusWrapper) -> Result<IscUInt64>
    {
        let result = unsafe { statement_get_affected_records(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_input_metadata(&self, status: &StatusWrapper) -> Result<MessageMetadata>
    {
        let result = unsafe { MessageMetadata{ this: statement_get_input_metadata(self.get_this(), status.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_output_metadata(&self, status: &StatusWrapper) -> Result<MessageMetadata>
    {
        let result = unsafe { MessageMetadata{ this: statement_get_output_metadata(self.get_this(), status.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn execute(&self, status: &StatusWrapper, transaction: &Transaction, in_metadata: &MessageMetadata, in_buffer: Ptr<Void>, out_metadata: &MessageMetadata, out_buffer: Ptr<Void>) -> Result<Transaction>
    {
        let result = unsafe { Transaction{ this: statement_execute(self.get_this(), status.this, transaction.this, in_metadata.this, in_buffer, out_metadata.this, out_buffer) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn open_cursor(&self, status: &StatusWrapper, transaction: &Transaction, in_metadata: &MessageMetadata, in_buffer: Ptr<Void>, out_metadata: &MessageMetadata, flags: UInt) -> Result<ResultSet>
    {
        let result = unsafe { ResultSet{ this: statement_open_cursor(self.get_this(), status.this, transaction.this, in_metadata.this, in_buffer, out_metadata.this, flags) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_cursor_name(&self, status: &StatusWrapper, name: CPtr<Char>) -> Result<()>
    {
        let result = unsafe { statement_set_cursor_name(self.get_this(), status.this, name) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn free(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { statement_free(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_flags(&self, status: &StatusWrapper) -> Result<UInt>
    {
        let result = unsafe { statement_get_flags(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(Statement, IReferenceCounted, IStatement);

pub trait IMessageMetadata : IReferenceCounted
{
    fn get_count(&self, status: &StatusWrapper) -> Result<UInt>
    {
        let result = unsafe { message_metadata_get_count(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_field(&self, status: &StatusWrapper, index: UInt) -> Result<CPtr<Char>>
    {
        let result = unsafe { message_metadata_get_field(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_relation(&self, status: &StatusWrapper, index: UInt) -> Result<CPtr<Char>>
    {
        let result = unsafe { message_metadata_get_relation(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_owner(&self, status: &StatusWrapper, index: UInt) -> Result<CPtr<Char>>
    {
        let result = unsafe { message_metadata_get_owner(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_alias(&self, status: &StatusWrapper, index: UInt) -> Result<CPtr<Char>>
    {
        let result = unsafe { message_metadata_get_alias(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_type(&self, status: &StatusWrapper, index: UInt) -> Result<UInt>
    {
        let result = unsafe { message_metadata_get_type(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn is_nullable(&self, status: &StatusWrapper, index: UInt) -> Result<FbBoolean>
    {
        let result = unsafe { message_metadata_is_nullable(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_sub_type(&self, status: &StatusWrapper, index: UInt) -> Result<Int>
    {
        let result = unsafe { message_metadata_get_sub_type(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_length(&self, status: &StatusWrapper, index: UInt) -> Result<UInt>
    {
        let result = unsafe { message_metadata_get_length(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_scale(&self, status: &StatusWrapper, index: UInt) -> Result<Int>
    {
        let result = unsafe { message_metadata_get_scale(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_char_set(&self, status: &StatusWrapper, index: UInt) -> Result<UInt>
    {
        let result = unsafe { message_metadata_get_char_set(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_offset(&self, status: &StatusWrapper, index: UInt) -> Result<UInt>
    {
        let result = unsafe { message_metadata_get_offset(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_null_offset(&self, status: &StatusWrapper, index: UInt) -> Result<UInt>
    {
        let result = unsafe { message_metadata_get_null_offset(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_builder(&self, status: &StatusWrapper) -> Result<MetadataBuilder>
    {
        let result = unsafe { MetadataBuilder{ this: message_metadata_get_builder(self.get_this(), status.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_message_length(&self, status: &StatusWrapper) -> Result<UInt>
    {
        let result = unsafe { message_metadata_get_message_length(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(MessageMetadata, IReferenceCounted, IMessageMetadata);

pub trait IMetadataBuilder : IReferenceCounted
{
    fn set_type(&self, status: &StatusWrapper, index: UInt, typ: UInt) -> Result<()>
    {
        let result = unsafe { metadata_builder_set_type(self.get_this(), status.this, index, typ) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_sub_type(&self, status: &StatusWrapper, index: UInt, sub_type: Int) -> Result<()>
    {
        let result = unsafe { metadata_builder_set_sub_type(self.get_this(), status.this, index, sub_type) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_length(&self, status: &StatusWrapper, index: UInt, length: UInt) -> Result<()>
    {
        let result = unsafe { metadata_builder_set_length(self.get_this(), status.this, index, length) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_char_set(&self, status: &StatusWrapper, index: UInt, char_set: UInt) -> Result<()>
    {
        let result = unsafe { metadata_builder_set_char_set(self.get_this(), status.this, index, char_set) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_scale(&self, status: &StatusWrapper, index: UInt, scale: Int) -> Result<()>
    {
        let result = unsafe { metadata_builder_set_scale(self.get_this(), status.this, index, scale) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn truncate(&self, status: &StatusWrapper, count: UInt) -> Result<()>
    {
        let result = unsafe { metadata_builder_truncate(self.get_this(), status.this, count) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn move_name_to_index(&self, status: &StatusWrapper, name: CPtr<Char>, index: UInt) -> Result<()>
    {
        let result = unsafe { metadata_builder_move_name_to_index(self.get_this(), status.this, name, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn remove(&self, status: &StatusWrapper, index: UInt) -> Result<()>
    {
        let result = unsafe { metadata_builder_remove(self.get_this(), status.this, index) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn add_field(&self, status: &StatusWrapper) -> Result<UInt>
    {
        let result = unsafe { metadata_builder_add_field(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_metadata(&self, status: &StatusWrapper) -> Result<MessageMetadata>
    {
        let result = unsafe { MessageMetadata{ this: metadata_builder_get_metadata(self.get_this(), status.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(MetadataBuilder, IReferenceCounted, IMetadataBuilder);

pub trait IEvents : IReferenceCounted
{

}

impl_as_def!(Events, IReferenceCounted, IEvents);

pub trait IEventCallback : IReferenceCounted
{

}

impl_as_def!(EventCallback, IReferenceCounted, IEventCallback);

pub trait IRequest : IReferenceCounted
{

}

impl_as_def!(Request, IReferenceCounted, IRequest);

pub trait IResultSet : IReferenceCounted
{
    fn fetch_next(&self, status: &StatusWrapper, message: Ptr<Void>) -> Result<Int>
    {
        let result = unsafe { result_set_fetch_next(self.get_this(), status.this, message) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn fetch_prior(&self, status: &StatusWrapper, message: Ptr<Void>) -> Result<Int>
    {
        let result = unsafe { result_set_fetch_prior(self.get_this(), status.this, message) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn fetch_first(&self, status: &StatusWrapper, message: Ptr<Void>) -> Result<Int>
    {
        let result = unsafe { result_set_fetch_first(self.get_this(), status.this, message) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn fetch_last(&self, status: &StatusWrapper, message: Ptr<Void>) -> Result<Int>
    {
        let result = unsafe { result_set_fetch_last(self.get_this(), status.this, message) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn fetch_absolute(&self, status: &StatusWrapper, position: Int, message: Ptr<Void>) -> Result<Int>
    {
        let result = unsafe { result_set_fetch_absolute(self.get_this(), status.this, position, message) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn fetch_relative(&self, status: &StatusWrapper, offset: Int, message: Ptr<Void>) -> Result<Int>
    {
        let result = unsafe { result_set_fetch_relative(self.get_this(), status.this, offset, message) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn is_eof(&self, status: &StatusWrapper) -> Result<FbBoolean>
    {
        let result = unsafe { result_set_is_eof(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn is_bof(&self, status: &StatusWrapper) -> Result<FbBoolean>
    {
        let result = unsafe { result_set_is_bof(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn get_metadata(&self, status: &StatusWrapper) -> Result<MessageMetadata>
    {
        let result = unsafe { MessageMetadata{ this: result_set_get_metadata(self.get_this(), status.this) } };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn close(&self, status: &StatusWrapper) -> Result<()>
    {
        let result = unsafe { result_set_close(self.get_this(), status.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
    fn set_delayed_output_format(&self, status: &StatusWrapper, format: &MessageMetadata) -> Result<()>
    {
        let result = unsafe { result_set_set_delayed_output_format(self.get_this(), status.this, format.this) };
        if status.has_data() == 1
        {
            return Err(Error::from_sw(&status));
        }
        Ok(result)
    }
}

impl_as_def!(ResultSet, IReferenceCounted, IResultSet);

pub trait IPluginManager : IVersioned
{

}

impl_as_def!(PluginManager, IVersioned, IPluginManager);

pub trait ITimerControl : IVersioned
{

}

impl_as_def!(TimerControl, IVersioned, ITimerControl);

pub trait IDtc : IVersioned
{

}

impl_as_def!(Dtc, IVersioned, IDtc);

pub trait IConfigManager : IVersioned
{

}

impl_as_def!(ConfigManager, IVersioned, IConfigManager);

pub trait IService : IReferenceCounted
{

}

impl_as_def!(Service, IReferenceCounted, IService);

pub trait ICryptKeyCallback : IVersioned
{

}

impl_as_def!(CryptKeyCallback, IVersioned, ICryptKeyCallback);

pub trait IVersionCallback : IVersioned
{

}

impl_as_def!(VersionCallback, IVersioned, IVersionCallback);

pub trait IOffsetsCallback : IVersioned
{

}

impl_as_def!(OffsetsCallback, IVersioned, IOffsetsCallback);



