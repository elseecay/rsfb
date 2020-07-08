pub type Ptr<T> = *mut T;
pub type CPtr<T> = *const T;
pub type Void = std::ffi::c_void;
pub type VoidPtr = Ptr<Void>;
pub type CVoidPtr = CPtr<Void>;
pub type Int = libc::c_int;
pub type UInt = libc::c_uint;
pub type Char = libc::c_schar;
pub type UChar = libc::c_uchar;
pub type Long = libc::c_longlong;
pub type ULong = libc::c_ulonglong;
pub type IntPtr = libc::intptr_t;
pub type UIntPtr = libc::uintptr_t;

pub type FbBoolean = UChar;





trait CxxClass
{
    fn get_this(&self) -> VoidPtr;
}

macro_rules! declare_cxx_class
{
    ($classname: ident, $ptrname: ident, $cptrname: ident) =>
    {
        pub type $ptrname = VoidPtr;
        pub type $cptrname = CVoidPtr;
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

declare_cxx_class!(Disposable, DisposablePtr, CDisposablePtr);
declare_cxx_class!(ReferenceCounted, ReferenceCountedPtr, CReferenceCountedPtr);
declare_cxx_class!(Master, MasterPtr, CMasterPtr);
declare_cxx_class!(Status, StatusPtr, CStatusPtr);
declare_cxx_class!(StatusWrapper, StatusWrapperPtr, CStatusWrapperPtr);
declare_cxx_class!(PluginBase, PluginBasePtr, CPluginBasePtr);
declare_cxx_class!(Provider, ProviderPtr, CProviderPtr);
declare_cxx_class!(Util, UtilPtr, CUtilPtr);
declare_cxx_class!(Attachment, AttachmentPtr, CAttachmentPtr);
declare_cxx_class!(XpbBuilder, XpbBuilderPtr, CXpbBuilderPtr);
declare_cxx_class!(Transaction, TransactionPtr, CTransactionPtr);
declare_cxx_class!(MessageMetadata, MessageMetadataPtr, CMessageMetadataPtr);
declare_cxx_class!(MetadataBuilder, MetadataBuilderPtr, CMetadataBuilderPtr);
declare_cxx_class!(Statement, StatementPtr, CStatementPtr);
declare_cxx_class!(Events, EventsPtr, CEventsPtr);
declare_cxx_class!(EventCallback, EventCallbackPtr, CEventCallbackPtr);
declare_cxx_class!(Request, RequestPtr, CRequestPtr);

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
    pub fn master_get_util_interface(this: MasterPtr) -> UtilPtr;

    // IStatus
    pub fn status_init(this: StatusPtr);
    pub fn status_set_errors2(this: StatusPtr, length: UInt, value: CPtr<IntPtr>);
    pub fn status_set_warnings2(this: StatusPtr, length: UInt, value: CPtr<IntPtr>);
    pub fn status_set_errors(this: StatusPtr, value: CPtr<IntPtr>);
    pub fn status_set_warnings(this: StatusPtr, value: CPtr<IntPtr>);

    // IProvider
    pub fn provider_create_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> AttachmentPtr;
    pub fn provider_attach_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> AttachmentPtr;

    // IUtil
    pub fn util_get_xpb_builder(this: UtilPtr, status: StatusWrapperPtr, kind: UInt, buf: CPtr<UChar>, len: UInt) -> XpbBuilderPtr;
    pub fn util_format_status(this: UtilPtr, buffer: Ptr<Char>, buffer_size: UInt, status: StatusWrapperPtr) -> UInt;

    // IXpbBuilder
    pub fn xpb_builder_insert_tag(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar);
    pub fn xpb_builder_insert_int(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, value: Int);
    pub fn xpb_builder_insert_string(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, str: CPtr<Char>);
    pub fn xpb_builder_get_buffer(this: XpbBuilderPtr, status: StatusWrapperPtr) -> CPtr<UChar>;
    pub fn xpb_builder_get_buffer_length(this: XpbBuilderPtr, status: StatusWrapperPtr) -> UInt;

    // IAttachment
    pub fn attachment_detach(this: AttachmentPtr, status: StatusWrapperPtr);
    pub fn attachment_prepare(this: AttachmentPtr, status: StatusWrapperPtr, tra: TransactionPtr, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, flags: UInt) -> StatementPtr;
    pub fn attachment_start_transaction(this: AttachmentPtr, status: StatusWrapperPtr, tpb_length: UInt, tpb: CPtr<UChar>) -> TransactionPtr;
    pub fn attachment_execute(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, stmt_length: UInt, sql_stmt: CPtr<Char>, dialect: UInt, in_metadata: MessageMetadataPtr, in_buffer: VoidPtr, out_metadata: MessageMetadataPtr, out_buffer: VoidPtr) -> TransactionPtr;
    pub fn attachment_get_info(this: AttachmentPtr, status: StatusWrapperPtr, items_length: UInt, items: CPtr<UChar>, buffer_length: UInt, buffer: Ptr<UChar>);
    pub fn attachment_reconnect_transaction(this: AttachmentPtr, status: StatusWrapperPtr, length: UInt, id: CPtr<UChar>) -> TransactionPtr;
    pub fn attachment_compile_request(this: AttachmentPtr, status: StatusWrapperPtr, blr_length: UInt, blr: CPtr<UChar>) -> RequestPtr;
    pub fn attachment_transact_request(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, blr_length: UInt, blr: CPtr<UChar>, in_msg_length: UInt, in_msg: CPtr<UChar>, out_msg_length: UInt, out_msg: Ptr<UChar>);
    pub fn attachment_execute_dyn(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, length: UInt, dyn: CPtr<UChar>);
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
    pub fn statement_get_output_metadata(this: StatementPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;

    // IMessageMetadata
    pub fn message_metadata_get_type(this: MessageMetadataPtr, status: StatusWrapperPtr, index: UInt) -> UInt;
    pub fn message_metadata_get_builder(this: MessageMetadataPtr, status: StatusWrapperPtr) -> MetadataBuilderPtr;
    pub fn message_metadata_get_count(this: MessageMetadataPtr, status: StatusWrapperPtr) -> UInt;

    // IMetadataBuilder
    pub fn metadata_builder_set_type(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: UInt, typ: UInt);
    pub fn metadata_builder_get_metadata(this: MetadataBuilderPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;
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
    fn on_drop(&mut self)
    {
        self.dispose()
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
    fn on_drop(&mut self)
    {
        self.release()
    }
}

pub trait IStatus : IDisposable
{
    fn init(&self)
    {
        unsafe { return status_init(self.get_this()); }
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
    fn get_util_interface(&self) -> Util
    {
        unsafe { return Util{ this: master_get_util_interface(self.get_this()) }; }
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
    fn get_xpb_builder(&self, status: &StatusWrapper, kind: UInt, buf: CPtr<UChar>, len: UInt) -> XpbBuilder
    {
        unsafe { return XpbBuilder{ this: util_get_xpb_builder(self.get_this(), status.this, kind, buf, len) }; }
    }
}

impl_as_def!(Util, IUtil);

pub trait IPluginBase : IReferenceCounted
{

}

pub trait IProvider : IPluginBase
{
    fn create_database(&self, status: &StatusWrapper, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> Attachment
    {
        unsafe { return Attachment{ this: provider_create_database(self.get_this(), status.this, file_name, dpb_length, dpb) }; }
    }
    fn attach_database(&self, status: &StatusWrapper, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> Attachment
    {
        unsafe { return Attachment{ this: provider_attach_database(self.get_this(), status.this, file_name, dpb_length, dpb) }; }
    }
}

impl_as_def!(Provider, IReferenceCounted, IPluginBase, IProvider);

pub trait IXpbBuilder : IDisposable
{
    fn insert_tag(&self, status: &StatusWrapper, tag: UChar)
    {
        unsafe { xpb_builder_insert_tag(self.get_this(), status.get_this(), tag); }
    }
    fn insert_int(&self, status: &StatusWrapper, tag: UChar, value: Int)
    {
        unsafe { xpb_builder_insert_int(self.get_this(), status.this, tag, value); }
    }
    fn insert_string(&self, status: &StatusWrapper, tag: UChar, str: CPtr<Char>)
    {
        unsafe { xpb_builder_insert_string(self.get_this(), status.this, tag, str); }
    }
    fn get_buffer(&self, status: &StatusWrapper) -> CPtr<UChar>
    {
        unsafe { return xpb_builder_get_buffer(self.get_this(), status.this); }
    }
    fn get_buffer_length(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return xpb_builder_get_buffer_length(self.get_this(), status.this); }
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
    fn execute_dyn(&self, status: &StatusWrapper, transaction: &Transaction, length: UInt, dyn: CPtr<UChar>)
    {
        unsafe { return attachment_execute_dyn(self.get_this(), status.this, transaction.this, length, dyn); }
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
    fn get_output_metadata(&self, status: &StatusWrapper) -> MessageMetadata
    {
        unsafe { return MessageMetadata{ this: statement_get_output_metadata(self.get_this(), status.this) }; }
    }
}

impl_as_def!(Statement, IReferenceCounted, IStatement);

pub trait IMessageMetadata : IReferenceCounted
{
    fn get_type(&self, status: &StatusWrapper, index: UInt) -> UInt
    {
        unsafe { return message_metadata_get_type(self.get_this(), status.this, index); }
    }
    fn get_builder(&self, status: &StatusWrapper) -> MetadataBuilder
    {
        unsafe { return MetadataBuilder{ this: message_metadata_get_builder(self.get_this(), status.this) }; }
    }
    fn get_count(&self, status: &StatusWrapper) -> UInt
    {
        unsafe { return message_metadata_get_count(self.get_this(), status.this); }
    }
}

impl_as_def!(MessageMetadata, IReferenceCounted, IMessageMetadata);

pub trait IMetadataBuilder : IReferenceCounted
{
    fn set_type(&self, status: &StatusWrapper, index: UInt, typ: UInt)
    {
       unsafe { metadata_builder_set_type(self.get_this(), status.this, index, typ); }
    }
    fn get_metadata(&self, status: &StatusWrapper) -> MessageMetadata
    {
       unsafe { return MessageMetadata{ this: metadata_builder_get_metadata(self.get_this(), status.this) } }
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



