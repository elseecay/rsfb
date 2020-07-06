pub type Ptr<T> = *mut T;
pub type CPtr<T> = *const T;
pub type Void = std::ffi::c_void;
pub type VoidPtr = Ptr<Void>;
pub type CVoidPtr = CPtr<Void>;
pub type Int = libc::c_int;
pub type UInt = libc::c_uint;
pub type Char = libc::c_schar;
pub type UChar = libc::c_uchar;

trait CxxClass
{
    fn get_this(&self) -> VoidPtr; // pointer mutable but struct not
    fn get_const_this(&self) -> CVoidPtr;
}

// rust marco cant concat like C macro $classname##Ptr
macro_rules! declare_cxx_class
{
    ($classname: ident, $ptrname: ident, $cptrname: ident) =>
    {
        pub type $ptrname = VoidPtr;
        pub type $cptrname = CVoidPtr;
        pub struct $classname
        {
            pub this: VoidPtr
        }
        impl CxxClass for $classname
        {
            fn get_this(&self) -> VoidPtr
            {
                self.this
            }
            fn get_const_this(&self) -> CVoidPtr
            {
                self.this
            }
        }
    };
}

declare_cxx_class!(Disposable, DisposablePtr, CDisposablePtr);
declare_cxx_class!(ReferenceCounted, ReferenceCountedPtr, CReferenceCountedPtr);
declare_cxx_class!(Master, MasterPtr, CMasterPtr);
declare_cxx_class!(Status, StatusPtr, CStatusPtr);
declare_cxx_class!(StatusWrapper, StatusWrapperPtr, CStatusWrapperPtr);
declare_cxx_class!(Provider, ProviderPtr, CProviderPtr);
declare_cxx_class!(Util, UtilPtr, CUtilPtr);
declare_cxx_class!(Attachment, AttachmentPtr, CAttachmentPtr);
declare_cxx_class!(XpbBuilder, XpbBuilderPtr, CXpbBuilderPtr);
declare_cxx_class!(Transaction, TransactionPtr, CTransactionPtr);
declare_cxx_class!(MessageMetadata, MessageMetadataPtr, CMessageMetadataPtr);
declare_cxx_class!(MetadataBuilder, MetadataBuilderPtr, CMetadataBuilderPtr);
declare_cxx_class!(Statement, StatementPtr, CStatementPtr);

#[link(name = "cfbapi")]
extern "C"
{
    // StatusWrapper
    pub fn status_wrapper_new(status: StatusPtr) -> StatusWrapperPtr;
    pub fn status_wrapper_free(this: StatusWrapperPtr);
    // END StatusWrapper

    // IDisposable
    pub fn disposable_dispose(this: DisposablePtr);
    // END IDisposable

    // IReferenceCounted
    pub fn reference_counted_add_ref(this: ReferenceCountedPtr);
    pub fn reference_counted_release(this: ReferenceCountedPtr) -> i32;
    // END IReferenceCounted

    // IMaster
    pub fn master_get_status(this: MasterPtr) -> StatusPtr;
    pub fn master_get_dispatcher(this: MasterPtr) -> ProviderPtr;
    pub fn master_get_util_interface(this: MasterPtr) -> UtilPtr;
    // END IMaster

    // IProvider
    pub fn provider_create_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> AttachmentPtr;
    pub fn provider_attach_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: CPtr<Char>, dpb_length: UInt, dpb: CPtr<UChar>) -> AttachmentPtr;
    // END IProvider

    // IUtil
    pub fn util_get_xpb_builder(this: UtilPtr, status: StatusWrapperPtr, kind: UInt, buf: CPtr<UChar>, len: UInt) -> XpbBuilderPtr;
    pub fn util_format_status(this: UtilPtr, buffer: *mut i8, buffer_size: u32, status: StatusWrapperPtr) -> u32;
    // END IUtil

    // IXpbBuilder
    pub fn xpb_builder_insert_tag(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar);
    pub fn xpb_builder_insert_int(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, value: Int);
    pub fn xpb_builder_insert_string(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: UChar, str: CPtr<Char>);
    pub fn xpb_builder_get_buffer(this: XpbBuilderPtr, status: StatusWrapperPtr) -> CPtr<UChar>;
    pub fn xpb_builder_get_buffer_length(this: XpbBuilderPtr, status: StatusWrapperPtr) -> UInt;
    // END IXpbBuilder

    // IAttachment
    pub fn attachment_detach(this: AttachmentPtr, status: StatusWrapperPtr);
    pub fn attachment_prepare(this: AttachmentPtr, status: StatusWrapperPtr, tra: TransactionPtr, stmt_length: u32, sql_stmt: *const i8, dialect: u32, flags: u32) -> StatementPtr;
    pub fn attachment_start_transaction(this: AttachmentPtr, status: StatusWrapperPtr, tpb_length: u32, tpb: *const u8) -> TransactionPtr;
    pub fn attachment_execute(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, stmt_length: u32, sql_stmt: *const i8, dialect: u32, in_metadata: MessageMetadataPtr, in_buffer: VoidPtr, out_metadata: MessageMetadataPtr, out_buffer: VoidPtr) -> TransactionPtr;
    // END IAttachment

    // ITransaction
    pub fn transaction_commit(this: TransactionPtr, status: StatusWrapperPtr);
    pub fn transaction_commit_retaining(this: TransactionPtr, status: StatusWrapperPtr);
    // END ITransaction

    // IStatement
    pub fn statement_get_output_metadata(this: StatementPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;
    // END IStatement

    // IMessageMetadata
    pub fn message_metadata_get_type(this: MessageMetadataPtr, status: StatusWrapperPtr, index: u32) -> u32;
    pub fn message_metadata_get_builder(this: MessageMetadataPtr, status: StatusWrapperPtr) -> MetadataBuilderPtr;
    pub fn message_metadata_get_count(this: MessageMetadataPtr, status: StatusWrapperPtr) -> u32;
    // END IMessageMetadata

    // IMetadataBuilder
    pub fn metadata_builder_set_type(this: MetadataBuilderPtr, status: StatusWrapperPtr, index: u32, typ: u32);
    pub fn metadata_builder_get_metadata(this: MetadataBuilderPtr, status: StatusWrapperPtr) -> MessageMetadataPtr;
    // END IMetadataBuilder
}

#[link(name = "fbclient")]
extern "C"
{
    pub fn fb_get_master_interface() -> MasterPtr;
}

pub trait IDisposable : CxxClass
{
    fn dispose(&self) { unsafe { disposable_dispose(self.get_this() as DisposablePtr); } }
}

pub trait IReferenceCounted : CxxClass
{
    fn add_ref(&self) { unsafe { reference_counted_add_ref(self.get_this() as ReferenceCountedPtr); } }
    fn release(&self) { unsafe { reference_counted_release(self.get_this() as ReferenceCountedPtr); } }
}

pub trait IStatus : IDisposable
{

}

impl IDisposable for Status { }
impl IStatus for Status { }

pub trait IMaster : CxxClass
{
    fn get() -> Master { unsafe { return Master{ this: fb_get_master_interface() }; } }
    fn get_status(&self) -> Status { unsafe { return Status{ this: master_get_status(self.get_this()) }; } }
    fn get_dispatcher(&self) -> Provider { unsafe { return Provider{ this: master_get_dispatcher(self.get_this()) }; } }
    fn get_util_interface(&self) -> Util { unsafe { return Util{ this: master_get_util_interface(self.get_this()) }; } }
}

impl IMaster for Master { }

pub trait IStatusWrapper : CxxClass
{
    fn new(status: &Status) -> StatusWrapper { unsafe { return StatusWrapper{ this: status_wrapper_new(status.get_this()) }; } }
    // TODO: delete
}

impl IStatusWrapper for StatusWrapper { }

pub trait IUtil : CxxClass
{
    fn get_xpb_builder(&self, status: &StatusWrapper, kind: UInt, buf: CPtr<UChar>, len: UInt) -> XpbBuilder
    {
        unsafe { return XpbBuilder{ this: util_get_xpb_builder(self.get_this(), status.get_this(), kind, buf, len) }; }
    }
}

impl IUtil for Util { }

pub trait IProvider : IReferenceCounted
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

impl IReferenceCounted for Provider { }
// TODO: IPluginBase
impl IProvider for Provider { }


pub trait IXpbBuilder : IDisposable
{
    fn insert_tag(&self, status: &StatusWrapper, tag: UChar)
    {
        unsafe { xpb_builder_insert_tag(self.get_this(), status.get_this(), tag); }
    }
    fn insert_int(&self, status: &StatusWrapper, tag: u8, value: Int)
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

impl IDisposable for XpbBuilder { }
impl IXpbBuilder for XpbBuilder { }

pub trait IAttachment : IReferenceCounted
{
    fn detach(&self, status: &StatusWrapper)
    {
        unsafe { attachment_detach(self.get_this(), status.this); }
    }
}

impl IReferenceCounted for Attachment { }
impl IAttachment for Attachment { }


