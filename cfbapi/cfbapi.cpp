#include <cstdlib>
#include <new>

#include <firebird/Interface.h>


using namespace Firebird;

using StatusWrapper = CheckStatusWrapper;

extern "C"
{
    // StatusWrapper
    StatusWrapper* status_wrapper_new(IStatus* status)
    {
        // TODO: check allocation error
        void* self = std::malloc(sizeof(StatusWrapper));
        new(self) StatusWrapper(status);
        return static_cast<StatusWrapper*>(self);
    }

    void status_wrapper_free(StatusWrapper* self)
    {
        self->~StatusWrapper();
        std::free(self);
    }
}

extern "C"
{
    // IDisposable 
    void disposable_dispose(IDisposable* self)
    {
        return self->dispose();
    }

    // IReferenceCounted 
    void reference_counted_add_ref(IReferenceCounted* self)
    {
        return self->addRef();
    }
    
    int reference_counted_release(IReferenceCounted* self)
    {
        return self->release();
    }
    // END IReferenceCounted 

    // IMaster 
    IStatus* master_get_status(IMaster* self)
    {
        return self->getStatus();
    }

    IProvider* master_get_dispatcher(IMaster* self)
    {
        return self->getDispatcher();
    }

    IUtil* master_get_util_interface(IMaster* self)
    {
        return self->getUtilInterface();
    }

    // IStatus
    void status_init(IStatus* self)
    {
        self->init();
    }
    void status_set_errors2(IStatus* self, unsigned length, const intptr_t* value)
    {
        self->setErrors2(length, value);
    }
    void status_set_warnings2(IStatus* self, unsigned length, const intptr_t* value)
    {
        self->setWarnings2(length, value);
    }
    void status_set_errors(IStatus* self, const intptr_t* value)
    {
        self->setErrors(value);
    }
    void status_set_warnings(IStatus* self, const intptr_t* value)
    {
        self->setWarnings(value);
    } // TODO: some const functions


    // IProvider 
    IAttachment* provider_create_database(IProvider* self, StatusWrapper* status, const char* fileName, unsigned dpbLength, const unsigned char* dpb)
    {
        return self->createDatabase(status, fileName, dpbLength, dpb);
    }

    IAttachment* provider_attach_database(IProvider* self, StatusWrapper* status, const char* fileName, unsigned dpbLength, const unsigned char* dpb)
    {
        return self->attachDatabase(status, fileName, dpbLength, dpb);
    }

    // IUtil 
    IXpbBuilder* util_get_xpb_builder(IUtil* self, StatusWrapper* status, unsigned kind, const unsigned char* buf, unsigned len)
    {
        return self->getXpbBuilder(status, kind, buf, len);
    }

    unsigned util_format_status(IUtil* self, char* buffer, unsigned bufferSize, IStatus* status)
    {
        return self->formatStatus(buffer, bufferSize, status);
    }

    // IXpbBuilder
    void xpb_builder_insert_tag(IXpbBuilder* self, StatusWrapper* status, unsigned char tag)
    {
        return self->insertTag(status, tag);
    } 

    void xpb_builder_insert_int(IXpbBuilder* self, StatusWrapper* status, unsigned char tag, int value)
    {
        return self->insertInt(status, tag, value);
    }

    void xpb_builder_insert_string(IXpbBuilder* self, StatusWrapper* status, unsigned char tag, const char* str)
    {
        return self->insertString(status, tag, str);
    }

    const unsigned char* xpb_builder_get_buffer(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getBuffer(status);
    }

    unsigned xpb_builder_get_buffer_length(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getBufferLength(status);
    }

    // IAttachment
    void attachment_detach(IAttachment* self, StatusWrapper* status)
    {
        return self->detach(status);
    }
    IStatement* attachment_prepare(IAttachment* self, StatusWrapper* status, ITransaction* tra, unsigned stmtLength, const char* sqlStmt, unsigned dialect, unsigned flags)
    {
        return self->prepare(status, tra, stmtLength, sqlStmt, dialect, flags);
    }
    ITransaction* attachment_start_transaction(IAttachment* self, StatusWrapper* status, unsigned tpbLength, const unsigned char* tpb)
    {
        return self->startTransaction(status, tpbLength, tpb);
    }
    ITransaction* attachment_execute(IAttachment* self, StatusWrapper* status, ITransaction* transaction, unsigned stmtLength, const char* sqlStmt, unsigned dialect, IMessageMetadata* inMetadata, void* inBuffer, IMessageMetadata* outMetadata, void* outBuffer)
    {
        return self->execute(status, transaction, stmtLength, sqlStmt, dialect, inMetadata, inBuffer, outMetadata, outBuffer);
    }
    void attachment_get_info(IAttachment* self, StatusType* status, unsigned itemsLength, const unsigned char* items, unsigned bufferLength, unsigned char* buffer)
    {
        self->getInfo(status, itemsLength, items, bufferLength, buffer);
    }
    ITransaction* attachment_reconnect_transaction(IAttachment* self, StatusType* status, unsigned length, const unsigned char* id)
    {
        self->reconnectTransaction(status, length, id);
    }
    IRequest* attachment_compile_request(IAttachment* self, StatusType* status, unsigned blrLength, const unsigned char* blr)
    {
        self->compileRequest(status, blrLength, blr);
    }
    void attachment_transact_request(IAttachment* self, StatusType* status, ITransaction* transaction, unsigned blrLength, const unsigned char* blr, unsigned inMsgLength, const unsigned char* inMsg, unsigned outMsgLength, unsigned char* outMsg)
    {
        self->transactRequest(status, transaction, blrLength, blr, inMsgLength, inMsg, outMsgLength, outMsg);
    }
    void attachment_execute_dyn(IAttachment* self, StatusType* status, ITransaction* transaction, unsigned length, const unsigned char* dyn)
    {
        self->executeDyn(status, transaction, length, dyn);
    }
    IResultSet* attachment_open_cursor(IAttachment* self, StatusType* status, ITransaction* transaction, unsigned stmtLength, const char* sqlStmt, unsigned dialect, IMessageMetadata* inMetadata, void* inBuffer, IMessageMetadata* outMetadata, const char* cursorName, unsigned cursorFlags)
    {
        self->openCursor(status, transaction, stmtLength, sqlStmt, dialect, inMetadata, inBuffer, outMetadata, cursorName, cursorFlags);
    }
    IEvents* attachment_que_events(IAttachment* self, StatusType* status, IEventCallback* callback, unsigned length, const unsigned char* events)
    {
        self->queEvents(status, callback, length, events);
    }
    void attachment_cancel_operation(IAttachment* self, StatusType* status, int option)
    {
        self->cancelOperation(status, option);
    }
    void attachment_ping(IAttachment* self, StatusType* status)
    {
        self->ping(status);
    }
    void attachment_drop_database(IAttachment* self, StatusType* status)
    {
        self->dropDatabase(status);
    }

    // ITransaction 
    void transaction_get_info(ITransaction* self, StatusType* status, unsigned itemsLength, const unsigned char* items, unsigned bufferLength, unsigned char* buffer)
    {
        self->getInfo(status, itemsLength, items, bufferLength, buffer);
    }
    void transaction_prepare(ITransaction* self, StatusType* status, unsigned msgLength, const unsigned char* message)
    {
        self->prepare(status, msgLength, message);
    }
    void transaction_commit(ITransaction* self, StatusType* status)
    {
        self->commit(status);
    }
    void transaction_commit_retaining(ITransaction* self, StatusType* status)
    {
        self->commitRetaining(status);
    }
    void transaction_rollback(ITransaction* self, StatusType* status)
    {
        self->rollback(status);
    }
    void transaction_rollback_retaining(ITransaction* self, StatusType* status)
    {
        self->rollbackRetaining(status);
    }
    void transaction_disconnect(ITransaction* self, StatusType* status)
    {
        self->disconnect(status);
    }
    ITransaction* transaction_join(ITransaction* self, StatusType* status, ITransaction* transaction)
    {
        self->join(status, transaction);
    }
    ITransaction* transaction_validate(ITransaction* self, StatusType* status, IAttachment* attachment)
    {
        self->validate(status, attachment);
    }
    ITransaction* transaction_enter_dtc(ITransaction* self, StatusType* status)
    {
        self->enterDtc(status);
    }

    // IStatement
    IMessageMetadata* statement_get_output_metadata(IStatement* self, StatusWrapper* status)
    {
        return self->getOutputMetadata(status);
    }

    // IMessageMetadata
    unsigned message_metadata_get_type(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getType(status, index);
    }

    IMetadataBuilder* message_metadata_get_builder(IMessageMetadata* self, StatusWrapper* status)
    {
        return self->getBuilder(status);
    }

    unsigned message_metadata_get_count(IMessageMetadata* self, StatusWrapper* status)
    {
        return self->getCount(status);
    }

    // IMetadataBuilder
    void metadata_builder_set_type(IMetadataBuilder* self, StatusWrapper* status, unsigned index, unsigned type)
    {
        return self->setType(status, index, type);
    }

    IMessageMetadata* metadata_builder_get_metadata(IMetadataBuilder* self, StatusWrapper* status)
    {
        return self->getMetadata(status);
    }
}