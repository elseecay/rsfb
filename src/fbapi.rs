pub type Ptr<T> = *mut T;
pub type CPtr<T> = *const T;
pub type Void = std::ffi::c_void;
pub type VoidPtr = Ptr<Void>;
pub type VoidCPtr = CPtr<Void>;
pub type Int = libc::c_int;
pub type UInt = libc::c_uint;
pub type Char = libc::c_schar;
pub type UChar = libc::c_uchar;
pub type Long = libc::c_longlong;
pub type ULong = libc::c_ulonglong;
pub type IntPtr = libc::intptr_t;
pub type UIntPtr = libc::uintptr_t;

pub type FbBoolean = UChar;
pub type IscInt64 = Long;
pub type IscUInt64 = ULong;
pub type IscDate = Int;
pub type IscTime = UInt;

#[cfg(target_pointer_width = "64")]
pub type IscLong = Int;
#[cfg(target_pointer_width = "64")]
pub type IscULong = UInt;

#[repr(C)]
struct GdsQuad
{
    quad_high: IscLong,
    quad_low: IscULong
}

pub type IscQuad = GdsQuad;





trait CxxClass
{
    fn get_this(&self) -> VoidPtr;
    fn get_cthis(&self) -> VoidCPtr;
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
    pub fn status_wrapper_free(this: StatusWrapperPtr);

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
    pub fn util_execute_create_database(this: UtilPtr, status: StatusWrapperPtr, stmt_length: UInt, creat_d_bstatement: CPtr<Char>, dialect: UInt, stmt_is_create_db: Ptr<FbBoolean>) -> AttachmentPtr;
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
    pub fn attachment_detach(this: AttachmentPtr, status: StatusWrapperPtr);
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
    pub fn transaction_commit(this: TransactionPtr, status: StatusWrapperPtr);
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

pub trait IDisposable : CxxClass
{
    fn dispose(&self)
    {
        unsafe { disposable_dispose(self.get_this() as DisposablePtr); }
    }
}

pub trait IReferenceCounted : CxxClass
{
    fn add_ref(&self)
    {
        unsafe { reference_counted_add_ref(self.get_this() as ReferenceCountedPtr); }
    }
    fn release(&self)
    {
        unsafe { reference_counted_release(self.get_this() as ReferenceCountedPtr); }
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

pub trait IMaster : CxxClass
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
    fn get_metadata_builder(&self, status: &StatusWrapper, field_count: UInt) -> MetadataBuilder
    {
        unsafe { return MetadataBuilder{ this: master_get_metadata_builder(self.get_this(), status.this, field_count) }; }
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

impl_as_def!(Master, IMaster);

pub trait IStatusWrapper : CxxClass
{
    fn new(status: &Status) -> StatusWrapper
    {
        unsafe { return StatusWrapper{ this: status_wrapper_new(status.get_this()) }; }
    }
    fn delete(&self)
    {
        unsafe { status_wrapper_free(self.get_this()); }
    }
}

impl_as_def!(StatusWrapper, IStatusWrapper);

pub trait IUtil : CxxClass
{
    fn get_fb_version(&self, status: &StatusWrapper, att: &Attachment, callback: &VersionCallback)
    {
        unsafe { return util_get_fb_version(self.get_this(), status.this, att.this, callback.this); }
    }
    fn load_blob(&self, status: &StatusWrapper, blob_id: Ptr<IscQuad>, att: &Attachment, tra: &Transaction, file: CPtr<Char>, txt: FbBoolean)
    {
        unsafe { return util_load_blob(self.get_this(), status.this, blob_id, att.this, tra.this, file, txt); }
    }
    fn dump_blob(&self, status: &StatusWrapper, blob_id: Ptr<IscQuad>, att: &Attachment, tra: &Transaction, file: CPtr<Char>, txt: FbBoolean)
    {
        unsafe { return util_dump_blob(self.get_this(), status.this, blob_id, att.this, tra.this, file, txt); }
    }
    fn get_perf_counters(&self, status: &StatusWrapper, att: &Attachment, counters_set: CPtr<Char>, counters: Ptr<IscInt64>)
    {
        unsafe { return util_get_perf_counters(self.get_this(), status.this, att.this, counters_set, counters); }
    }
    fn execute_create_database(&self, status: &StatusWrapper, stmt_length: UInt, creat_d_bstatement: CPtr<Char>, dialect: UInt, stmt_is_create_db: Ptr<FbBoolean>) -> Attachment
    {
        unsafe { return Attachment{ this: util_execute_create_database(self.get_this(), status.this, stmt_length, creat_d_bstatement, dialect, stmt_is_create_db) }; }
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
    fn get_xpb_builder(&self, status: &StatusWrapper, kind: UInt, buf: CPtr<UChar>, len: UInt) -> XpbBuilder
    {
        unsafe { return XpbBuilder{ this: util_get_xpb_builder(self.get_this(), status.this, kind, buf, len) }; }
    }
    fn set_offsets(&self, status: &StatusWrapper, metadata: &MessageMetadata, callback: &OffsetsCallback) -> UInt
    {
        unsafe { return util_set_offsets(self.get_this(), status.this, metadata.this, callback.this); }
    }
}

impl_as_def!(Util, IUtil);

pub trait IPluginBase : IReferenceCounted
{

}

pub trait IProvider : IPluginBase
{
    fn shutdown(&self, status: &StatusWrapper, timeout: UInt, reason: Int)
    {
        unsafe { provider_shutdown(self.get_this(), status.this, timeout, reason); }
    }
    fn attach_database(&self, status: &StatusWrapper, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> Attachment
    {
        unsafe { return Attachment{ this: provider_attach_database(self.get_this(), status.this, file_name, dpb_length, dpb) }; }
    }
    fn create_database(&self, status: &StatusWrapper, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> Attachment
    {
        unsafe { return Attachment{ this: provider_create_database(self.get_this(), status.this, file_name, dpb_length, dpb) }; }
    }
    fn attach_service_manager(&self, status: &StatusWrapper, service: CPtr<Char>, spb_length: UInt, spb: CPtr<UChar>) -> Service
    {
        unsafe { return Service{ this: provider_attach_service_manager(self.get_this(), status.this, service, spb_length, spb) }; }
    }
    fn set_db_crypt_callback(&self, status: &StatusWrapper, crypt_callback: &CryptKeyCallback)
    {
        unsafe { return provider_set_db_crypt_callback(self.get_this(), status.this, crypt_callback.this); }
    }
}

impl_as_def!(Provider, IReferenceCounted, IPluginBase, IProvider);

pub trait IXpbBuilder : IDisposable
{
    fn clear(&self, status: &StatusWrapper)
    {
        unsafe { return xpb_builder_clear(self.get_this(), status.this); }
    }
    fn remove_current(&self, status: &StatusWrapper)
    {
        unsafe { return xpb_builder_remove_current(self.get_this(), status.this); }
    }
    fn insert_int(&self, status: &StatusWrapper, tag: UChar, value: Int)
    {
        unsafe { return xpb_builder_insert_int(self.get_this(), status.this, tag, value); }
    }
    fn insert_big_int(&self, status: &StatusWrapper, tag: UChar, value: IscInt64)
    {
        unsafe { return xpb_builder_insert_big_int(self.get_this(), status.this, tag, value); }
    }
    fn insert_bytes(&self, status: &StatusWrapper, tag: UChar, bytes: CPtr<Void>, length: UInt)
    {
        unsafe { return xpb_builder_insert_bytes(self.get_this(), status.this, tag, bytes, length); }
    }
    fn insert_string(&self, status: &StatusWrapper, tag: UChar, str: CPtr<Char>)
    {
        unsafe { return xpb_builder_insert_string(self.get_this(), status.this, tag, str); }
    }
    fn insert_tag(&self, status: &StatusWrapper, tag: UChar)
    {
        unsafe { return xpb_builder_insert_tag(self.get_this(), status.this, tag); }
    }
    fn is_eof(&self, status: &StatusWrapper) -> FbBoolean
    {
        unsafe { return xpb_builder_is_eof(self.get_this(), status.this); }
    }
    fn move_next(&self, status: &StatusWrapper)
    {
        unsafe { return xpb_builder_move_next(self.get_this(), status.this); }
    }
    fn rewind(&self, status: &StatusWrapper)
    {
        unsafe { return xpb_builder_rewind(self.get_this(), status.this); }
    }
    fn find_first(&self, status: &StatusWrapper, tag: UChar) -> FbBoolean
    {
        unsafe { return xpb_builder_find_first(self.get_this(), status.this, tag); }
    }
    fn find_next(&self, status: &StatusWrapper) -> FbBoolean
    {
        unsafe { return xpb_builder_find_next(self.get_this(), status.this); }
    }
    fn get_tag(&self, status: &StatusWrapper) -> UChar
    {
        unsafe { return xpb_builder_get_tag(self.get_this(), status.this); }
    }
    fn get_length(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return xpb_builder_get_length(self.get_this(), status.this); }
    }
    fn get_int(&self, status: &StatusWrapper) -> Int
    {
        unsafe { return xpb_builder_get_int(self.get_this(), status.this); }
    }
    fn get_big_int(&self, status: &StatusWrapper) -> IscInt64
    {
        unsafe { return xpb_builder_get_big_int(self.get_this(), status.this); }
    }
    fn get_string(&self, status: &StatusWrapper) -> CPtr<Char>
    {
        unsafe { return xpb_builder_get_string(self.get_this(), status.this); }
    }
    fn get_bytes(&self, status: &StatusWrapper) -> CPtr<UChar>
    {
        unsafe { return xpb_builder_get_bytes(self.get_this(), status.this); }
    }
    fn get_buffer_length(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return xpb_builder_get_buffer_length(self.get_this(), status.this); }
    }
    fn get_buffer(&self, status: &StatusWrapper) -> CPtr<UChar>
    {
        unsafe { return xpb_builder_get_buffer(self.get_this(), status.this); }
    }
}

impl_as_def!(XpbBuilder, IDisposable, IXpbBuilder);

pub trait IAttachment : IReferenceCounted
{
    fn detach(&self, status: &StatusWrapper)
    {
        unsafe { attachment_detach(self.get_this(), status.this); }
    }
    fn prepare(&self, status: &StatusWrapper, tra: &Transaction, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, flags: UInt) -> Statement
    {
        unsafe { return Statement{ this: attachment_prepare(self.get_this(), status.this, tra.this, stmt_length, sql_stmt, dialect, flags) }; }
    }
    fn start_transaction(&self, status: &StatusWrapper, tpb_length: UInt, tpb: CPtr<UChar>) -> Transaction
    {
        unsafe { return Transaction{ this: attachment_start_transaction(self.get_this(), status.this, tpb_length, tpb) }; }
    }
    fn execute(&self, status: &StatusWrapper, transaction: &Transaction, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, in_metadata: &MessageMetadata, in_buffer: VoidPtr, out_metadata: &MessageMetadata, out_buffer: VoidPtr) -> Transaction
    {
        unsafe { return Transaction{ this: attachment_execute(self.get_this(), status.this, transaction.this, stmt_length, sql_stmt, dialect, in_metadata.this, in_buffer, out_metadata.this, out_buffer) }; }
    }
    fn get_info(&self, status: &StatusWrapper, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>)
    {
        unsafe { return attachment_get_info(self.get_this(), status.this, items_length, items, buffer_length, buffer); }
    }
    fn reconnect_transaction(&self, status: &StatusWrapper, length: UInt, id: CPtr<UChar>) -> Transaction
    {
        unsafe { return Transaction{ this: attachment_reconnect_transaction(self.get_this(), status.this, length, id) }; }
    }
    fn compile_request(&self, status: &StatusWrapper, blr_length: UInt, blr: CPtr<UChar>) -> Request
    {
        unsafe { return Request{ this: attachment_compile_request(self.get_this(), status.this, blr_length, blr) }; }
    }
    fn transact_request(&self, status: &StatusWrapper, transaction: &Transaction, blr_length: UInt, blr: CPtr<UChar>, in_msg_length: UInt, in_msg: CPtr<UChar>, out_msg_length: UInt, out_msg: Ptr<UChar>)
    {
        unsafe { return attachment_transact_request(self.get_this(), status.this, transaction.this, blr_length, blr, in_msg_length, in_msg, out_msg_length, out_msg); }
    }
    fn execute_dyn(&self, status: &StatusWrapper, transaction: &Transaction, length: UInt, dn: CPtr<UChar>)
    {
        unsafe { return attachment_execute_dyn(self.get_this(), status.this, transaction.this, length, dn); }
    }
    fn open_cursor(&self, status: &StatusWrapper, transaction: &Transaction, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, in_metadata: &MessageMetadata, in_buffer: Ptr<Void>, out_metadata: &MessageMetadata, cursor_name: CPtr<Char>, cursor_flags: UInt) -> ResultSet
    {
        unsafe { return ResultSet{ this: attachment_open_cursor(self.get_this(), status.this, transaction.this, stmt_length, sql_stmt, dialect, in_metadata.this, in_buffer, out_metadata.this, cursor_name, cursor_flags) }; }
    }
    fn que_events(&self, status: &StatusWrapper, callback: &EventCallback, length: UInt, events: CPtr<UChar>) -> Events
    {
        unsafe { return Events{ this: attachment_que_events(self.get_this(), status.this, callback.this, length, events) }; }
    }
    fn cancel_operation(&self, status: &StatusWrapper, option: Int)
    {
        unsafe { return attachment_cancel_operation(self.get_this(), status.this, option); }
    }
    fn ping(&self, status: &StatusWrapper)
    {
        unsafe { return attachment_ping(self.get_this(), status.this); }
    }
    fn drop_database(&self, status: &StatusWrapper)
    {
        unsafe { return attachment_drop_database(self.get_this(), status.this); }
    }
}

impl_as_def!(Attachment, IReferenceCounted, IAttachment);

pub trait ITransaction : IReferenceCounted
{
    fn get_info(&self, status: &StatusWrapper, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>)
    {
        unsafe { return transaction_get_info(self.get_this(), status.this, items_length, items, buffer_length, buffer); }
    }
    fn prepare(&self, status: &StatusWrapper, msg_length: UInt, message: CPtr<UChar>)
    {
        unsafe { return transaction_prepare(self.get_this(), status.this, msg_length, message); }
    }
    fn commit(&self, status: &StatusWrapper)
    {
        unsafe { return transaction_commit(self.get_this(), status.this); }
    }
    fn commit_retaining(&self, status: &StatusWrapper)
    {
        unsafe { return transaction_commit_retaining(self.get_this(), status.this); }
    }
    fn rollback(&self, status: &StatusWrapper)
    {
        unsafe { return transaction_rollback(self.get_this(), status.this); }
    }
    fn rollback_retaining(&self, status: &StatusWrapper)
    {
        unsafe { return transaction_rollback_retaining(self.get_this(), status.this); }
    }
    fn disconnect(&self, status: &StatusWrapper)
    {
        unsafe { return transaction_disconnect(self.get_this(), status.this); }
    }
    fn join(&self, status: &StatusWrapper, transaction: &Transaction) -> Transaction
    {
        unsafe { return Transaction{ this: transaction_join(self.get_this(), status.this, transaction.this) }; }
    }
    fn validate(&self, status: &StatusWrapper, attachment: &Attachment) -> Transaction
    {
        unsafe { return Transaction{ this: transaction_validate(self.get_this(), status.this, attachment.this) }; }
    }
    fn enter_dtc(&self, status: &StatusWrapper) -> Transaction
    {
        unsafe { return Transaction{ this: transaction_enter_dtc(self.get_this(), status.this) }; }
    }
}

impl_as_def!(Transaction, IReferenceCounted, ITransaction);

pub trait IStatement : IReferenceCounted
{
    fn get_info(&self, status: &StatusWrapper, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>)
    {
        unsafe { return statement_get_info(self.get_this(), status.this, items_length, items, buffer_length, buffer); }
    }
    fn get_type(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return statement_get_type(self.get_this(), status.this); }
    }
    fn get_plan(&self, status: &StatusWrapper, detailed: FbBoolean) -> CPtr<Char>
    {
        unsafe { return statement_get_plan(self.get_this(), status.this, detailed); }
    }
    fn get_affected_records(&self, status: &StatusWrapper) -> IscUInt64
    {
        unsafe { return statement_get_affected_records(self.get_this(), status.this); }
    }
    fn get_input_metadata(&self, status: &StatusWrapper) -> MessageMetadata
    {
        unsafe { return MessageMetadata{ this: statement_get_input_metadata(self.get_this(), status.this) }; }
    }
    fn get_output_metadata(&self, status: &StatusWrapper) -> MessageMetadata
    {
        unsafe { return MessageMetadata{ this: statement_get_output_metadata(self.get_this(), status.this) }; }
    }
    fn execute(&self, status: &StatusWrapper, transaction: &Transaction, in_metadata: &MessageMetadata, in_buffer: Ptr<Void>, out_metadata: &MessageMetadata, out_buffer: Ptr<Void>) -> Transaction
    {
        unsafe { return Transaction{ this: statement_execute(self.get_this(), status.this, transaction.this, in_metadata.this, in_buffer, out_metadata.this, out_buffer) }; }
    }
    fn open_cursor(&self, status: &StatusWrapper, transaction: &Transaction, in_metadata: &MessageMetadata, in_buffer: Ptr<Void>, out_metadata: &MessageMetadata, flags: UInt) -> ResultSet
    {
        unsafe { return ResultSet{ this: statement_open_cursor(self.get_this(), status.this, transaction.this, in_metadata.this, in_buffer, out_metadata.this, flags) }; }
    }
    fn set_cursor_name(&self, status: &StatusWrapper, name: CPtr<Char>)
    {
        unsafe { return statement_set_cursor_name(self.get_this(), status.this, name); }
    }
    fn free(&self, status: &StatusWrapper)
    {
        unsafe { return statement_free(self.get_this(), status.this); }
    }
    fn get_flags(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return statement_get_flags(self.get_this(), status.this); }
    }
}

impl_as_def!(Statement, IReferenceCounted, IStatement);

pub trait IMessageMetadata : IReferenceCounted
{
    fn get_count(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return message_metadata_get_count(self.get_this(), status.this); }
    }
    fn get_field(&self, status: &StatusWrapper, index: UInt) -> CPtr<Char>
    {
        unsafe { return message_metadata_get_field(self.get_this(), status.this, index); }
    }
    fn get_relation(&self, status: &StatusWrapper, index: UInt) -> CPtr<Char>
    {
        unsafe { return message_metadata_get_relation(self.get_this(), status.this, index); }
    }
    fn get_owner(&self, status: &StatusWrapper, index: UInt) -> CPtr<Char>
    {
        unsafe { return message_metadata_get_owner(self.get_this(), status.this, index); }
    }
    fn get_alias(&self, status: &StatusWrapper, index: UInt) -> CPtr<Char>
    {
        unsafe { return message_metadata_get_alias(self.get_this(), status.this, index); }
    }
    fn get_type(&self, status: &StatusWrapper, index: UInt) -> UInt
    {
        unsafe { return message_metadata_get_type(self.get_this(), status.this, index); }
    }
    fn is_nullable(&self, status: &StatusWrapper, index: UInt) -> FbBoolean
    {
        unsafe { return message_metadata_is_nullable(self.get_this(), status.this, index); }
    }
    fn get_sub_type(&self, status: &StatusWrapper, index: UInt) -> Int
    {
        unsafe { return message_metadata_get_sub_type(self.get_this(), status.this, index); }
    }
    fn get_length(&self, status: &StatusWrapper, index: UInt) -> UInt
    {
        unsafe { return message_metadata_get_length(self.get_this(), status.this, index); }
    }
    fn get_scale(&self, status: &StatusWrapper, index: UInt) -> Int
    {
        unsafe { return message_metadata_get_scale(self.get_this(), status.this, index); }
    }
    fn get_char_set(&self, status: &StatusWrapper, index: UInt) -> UInt
    {
        unsafe { return message_metadata_get_char_set(self.get_this(), status.this, index); }
    }
    fn get_offset(&self, status: &StatusWrapper, index: UInt) -> UInt
    {
        unsafe { return message_metadata_get_offset(self.get_this(), status.this, index); }
    }
    fn get_null_offset(&self, status: &StatusWrapper, index: UInt) -> UInt
    {
        unsafe { return message_metadata_get_null_offset(self.get_this(), status.this, index); }
    }
    fn get_builder(&self, status: &StatusWrapper) -> MetadataBuilder
    {
        unsafe { return MetadataBuilder{ this: message_metadata_get_builder(self.get_this(), status.this) }; }
    }
    fn get_message_length(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return message_metadata_get_message_length(self.get_this(), status.this); }
    }
}

impl_as_def!(MessageMetadata, IReferenceCounted, IMessageMetadata);

pub trait IMetadataBuilder : IReferenceCounted
{
    fn set_type(&self, status: &StatusWrapper, index: UInt, typ: UInt)
    {
        unsafe { return metadata_builder_set_type(self.get_this(), status.this, index, typ); }
    }
    fn set_sub_type(&self, status: &StatusWrapper, index: UInt, sub_type: Int)
    {
        unsafe { return metadata_builder_set_sub_type(self.get_this(), status.this, index, sub_type); }
    }
    fn set_length(&self, status: &StatusWrapper, index: UInt, length: UInt)
    {
        unsafe { return metadata_builder_set_length(self.get_this(), status.this, index, length); }
    }
    fn set_char_set(&self, status: &StatusWrapper, index: UInt, char_set: UInt)
    {
        unsafe { return metadata_builder_set_char_set(self.get_this(), status.this, index, char_set); }
    }
    fn set_scale(&self, status: &StatusWrapper, index: UInt, scale: Int)
    {
        unsafe { return metadata_builder_set_scale(self.get_this(), status.this, index, scale); }
    }
    fn truncate(&self, status: &StatusWrapper, count: UInt)
    {
        unsafe { return metadata_builder_truncate(self.get_this(), status.this, count); }
    }
    fn move_name_to_index(&self, status: &StatusWrapper, name: CPtr<Char>, index: UInt)
    {
        unsafe { return metadata_builder_move_name_to_index(self.get_this(), status.this, name, index); }
    }
    fn remove(&self, status: &StatusWrapper, index: UInt)
    {
        unsafe { return metadata_builder_remove(self.get_this(), status.this, index); }
    }
    fn add_field(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return metadata_builder_add_field(self.get_this(), status.this); }
    }
    fn get_metadata(&self, status: &StatusWrapper) -> MessageMetadata
    {
        unsafe { return MessageMetadata{ this: metadata_builder_get_metadata(self.get_this(), status.this) }; }
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
    fn fetch_next(&self, status: &StatusWrapper, message: Ptr<Void>) -> Int
    {
        unsafe { return result_set_fetch_next(self.get_this(), status.this, message); }
    }
    fn fetch_prior(&self, status: &StatusWrapper, message: Ptr<Void>) -> Int
    {
        unsafe { return result_set_fetch_prior(self.get_this(), status.this, message); }
    }
    fn fetch_first(&self, status: &StatusWrapper, message: Ptr<Void>) -> Int
    {
        unsafe { return result_set_fetch_first(self.get_this(), status.this, message); }
    }
    fn fetch_last(&self, status: &StatusWrapper, message: Ptr<Void>) -> Int
    {
        unsafe { return result_set_fetch_last(self.get_this(), status.this, message); }
    }
    fn fetch_absolute(&self, status: &StatusWrapper, position: Int, message: Ptr<Void>) -> Int
    {
        unsafe { return result_set_fetch_absolute(self.get_this(), status.this, position, message); }
    }
    fn fetch_relative(&self, status: &StatusWrapper, offset: Int, message: Ptr<Void>) -> Int
    {
        unsafe { return result_set_fetch_relative(self.get_this(), status.this, offset, message); }
    }
    fn is_eof(&self, status: &StatusWrapper) -> FbBoolean
    {
        unsafe { return result_set_is_eof(self.get_this(), status.this); }
    }
    fn is_bof(&self, status: &StatusWrapper) -> FbBoolean
    {
        unsafe { return result_set_is_bof(self.get_this(), status.this); }
    }
    fn get_metadata(&self, status: &StatusWrapper) -> MessageMetadata
    {
        unsafe { return MessageMetadata{ this: result_set_get_metadata(self.get_this(), status.this) }; }
    }
    fn close(&self, status: &StatusWrapper)
    {
        unsafe { return result_set_close(self.get_this(), status.this); }
    }
    fn set_delayed_output_format(&self, status: &StatusWrapper, format: &MessageMetadata)
    {
        unsafe { return result_set_set_delayed_output_format(self.get_this(), status.this, format.this); }
    }
}

impl_as_def!(ResultSet, IReferenceCounted, IResultSet);



