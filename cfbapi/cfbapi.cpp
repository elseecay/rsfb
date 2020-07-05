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
    // END StatusWrapper
}

extern "C"
{
    // IDisposable 
    void disposable_dispose(IDisposable* self)
    {
        return self->dispose();
    }
    // END IDisposable 

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
    // END IMaster 

    // IProvider 
    IAttachment* provider_create_database(IProvider* self, StatusWrapper* status, const char* fileName, unsigned dpbLength, const unsigned char* dpb)
    {
        return self->createDatabase(status, fileName, dpbLength, dpb);
    }

    IAttachment* provider_attach_database(IProvider* self, StatusWrapper* status, const char* fileName, unsigned dpbLength, const unsigned char* dpb)
    {
        return self->attachDatabase(status, fileName, dpbLength, dpb);
    }
    // END IProvider 

    // IUtil 
    IXpbBuilder* util_get_xpb_builder(IUtil* self, StatusWrapper* status, unsigned kind, const unsigned char* buf, unsigned len)
    {
        return self->getXpbBuilder(status, kind, buf, len);
    }

    unsigned util_format_status(IUtil* self, char* buffer, unsigned bufferSize, IStatus* status)
    {
        return self->formatStatus(buffer, bufferSize, status);
    }
    // END IUtil 

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
    // END IXpbBuilder 

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
    // END IAttachment 

    // ITransaction 
    void transaction_commit(ITransaction* self, StatusWrapper* status)
    {
        return self->commit(status);
    }

    void transaction_commit_retaining(ITransaction* self, StatusWrapper* status)
    {
        return self->commitRetaining(status);
    }
    // END ITransaction 

    // IStatement
    IMessageMetadata* statement_get_output_metadata(IStatement* self, StatusWrapper* status)
    {
        return self->getOutputMetadata(status);
    }
    // END IStatement

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
    // END IMessageMetadata

    // IMetadataBuilder
    void metadata_builder_set_type(IMetadataBuilder* self, StatusWrapper* status, unsigned index, unsigned type)
    {
        return self->setType(status, index, type);
    }

    IMessageMetadata* metadata_builder_get_metadata(IMetadataBuilder* self, StatusWrapper* status)
    {
        return self->getMetadata(status);
    }
    // END IMetadataBuilder
}