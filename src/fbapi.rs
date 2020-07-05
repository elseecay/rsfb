use std::ffi::c_void;

pub type Void = c_void;
pub type VoidPtr = *mut Void;
pub type CVoidPtr = *const Void;
pub type MasterPtr = VoidPtr;
pub type CMasterPtr = CVoidPtr;
pub type DisposablePtr = VoidPtr;
pub type CDisposablePtr = CVoidPtr;
pub type ReferenceCountedPtr = VoidPtr;
pub type CReferenceCountedPtr = CVoidPtr;
pub type StatusPtr = VoidPtr;
pub type CStatusPtr = CVoidPtr;
pub type StatusWrapperPtr = VoidPtr;
pub type ProviderPtr = VoidPtr;
pub type CProviderPtr = CVoidPtr;
pub type UtilPtr = VoidPtr;
pub type CUtilPtr = CVoidPtr;
pub type AttachmentPtr = VoidPtr;
pub type CAttachmentPtr = CVoidPtr;
pub type XpbBuilderPtr = VoidPtr;
pub type CXpbBuilderPtr = CVoidPtr;
pub type TransactionPtr = VoidPtr;
pub type CTransactionPtr = CVoidPtr;
pub type MessageMetadataPtr = VoidPtr;
pub type CMessageMetadataPtr = CVoidPtr;
pub type MetadataBuilderPtr = VoidPtr;
pub type CMetadataBuilderPtr = CVoidPtr;
pub type StatementPtr = VoidPtr;
pub type CStatementPtr = CVoidPtr;


// TODO: may be replace i8 ptrs with CStr
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
    pub fn provider_create_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: *const i8, dpb_length: u32, dpb: *const u8) -> AttachmentPtr;
    pub fn provider_attach_database(this: ProviderPtr, status: StatusWrapperPtr, file_name: *const i8, dpb_length: u32, dpb: *const u8) -> AttachmentPtr;
    // END IProvider

    // IUtil
    pub fn util_get_xpb_builder(this: UtilPtr, status: StatusWrapperPtr, kind: u32, buf: *const u8, len: u32) -> XpbBuilderPtr;
    pub fn util_format_status(this: UtilPtr, buffer: *mut i8, buffer_size: u32, status: StatusWrapperPtr) -> u32;
    // END IUtil

    // IXpbBuilder
    pub fn xpb_builder_insert_tag(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: u8);
    pub fn xpb_builder_insert_int(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: u8, value: i32);
    pub fn xpb_builder_insert_string(this: XpbBuilderPtr, status: StatusWrapperPtr, tag: u8, str: *const i8);
    pub fn xpb_builder_get_buffer(this: XpbBuilderPtr, status: StatusWrapperPtr) -> *const u8;
    pub fn xpb_builder_get_buffer_length(this: XpbBuilderPtr, status: StatusWrapperPtr) -> u32;
    // END IXpbBuilder

    // IAttachment
    pub fn attachment_detach(this: AttachmentPtr, status: StatusWrapperPtr);
    pub fn attachment_prepare(this: AttachmentPtr, status: StatusWrapperPtr, tra: TransactionPtr, stmt_length: u32, sql_stmt: *const i8, dialect: u32, flags: u32) -> StatementPtr;
    pub fn attachment_start_transaction(this: AttachmentPtr, status: StatusWrapperPtr, tpb_length: u32, tpb: *const u8) -> TransactionPtr;
    pub fn attachment_execute(this: AttachmentPtr, status: StatusWrapperPtr, transaction: TransactionPtr, stmt_length: u32, sql_stmt: *const i8, dialect: u32, in_metadata: MessageMetadataPtr, in_buffer: *mut c_void, out_metadata: MessageMetadataPtr, out_buffer: *mut c_void) -> TransactionPtr;
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

pub trait CxxClass
{
    fn get_this(&mut self) -> VoidPtr;
    fn get_const_this(&self) -> CVoidPtr;
}

pub trait Disposable : CxxClass
{
    fn dispose(&mut self)
    {
        unsafe { disposable_dispose(self.get_this() as DisposablePtr); }
    }
}

pub trait ReferenceCounted : CxxClass
{
    fn add_ref(&mut self)
    {
        unsafe { reference_counted_add_ref(self.get_this() as ReferenceCountedPtr); }
    }
    fn release(&mut self)
    {
        unsafe { reference_counted_release(self.get_this() as ReferenceCountedPtr); }
    }
}

macro_rules! declare_cxx_class
{
    ($classname: ident) =>
    {
        pub struct $classname
        {
            pub this: VoidPtr
        }
        impl CxxClass for $classname
        {
            fn get_this(&mut self) -> VoidPtr
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

declare_cxx_class!(Master);
declare_cxx_class!(Status);
declare_cxx_class!(StatusWrapper);
declare_cxx_class!(Provider);
declare_cxx_class!(Util);


impl Master
{
    pub fn new() -> Master
    {
        unsafe
        {
            return Master{ this: fb_get_master_interface() };
        }
    }
    pub fn get_status(&mut self) -> Status
    {
        unsafe
        {
            return Status { this: master_get_status(self.this) };
        }
    }
    pub fn get_dispatcher(&mut self) -> Provider
    {
        unsafe
        {
            return Provider{ this: master_get_dispatcher(self.this) };
        }
    }
    pub fn get_util_interface(&mut self) -> Util
    {
        unsafe
        {
            return Util{ this: master_get_util_interface(self.this) };
        }
    }
    // pub fn master_get_status(this: MasterPtr) -> StatusPtr;
    // pub fn master_get_dispatcher(this: MasterPtr) -> ProviderPtr;
    // pub fn master_get_util_interface(this: MasterPtr) -> UtilPtr;
}
