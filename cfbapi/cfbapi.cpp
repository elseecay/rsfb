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
    IPluginManager* master_get_plugin_manager(IMaster* self)
    {
        return self->getPluginManager();
    }
    ITimerControl* master_get_timer_control(IMaster* self)
    {
        return self->getTimerControl();
    }
    IDtc* master_get_dtc(IMaster* self)
    {
        return self->getDtc();
    }
    IAttachment* master_register_attachment(IMaster* self, IProvider* provider, IAttachment* attachment)
    {
        return self->registerAttachment(provider, attachment);
    }
    ITransaction* master_register_transaction(IMaster* self, IAttachment* attachment, ITransaction* transaction)
    {
        return self->registerTransaction(attachment, transaction);
    }
    IMetadataBuilder* master_get_metadata_builder(IMaster* self, StatusWrapper* status, unsigned fieldCount)
    {
        return self->getMetadataBuilder(status, fieldCount);
    }
    int master_server_mode(IMaster* self, int mode)
    {
        return self->serverMode(mode);
    }
    IUtil* master_get_util_interface(IMaster* self)
    {
        return self->getUtilInterface();
    }
    IConfigManager* master_get_config_manager(IMaster* self)
    {
        return self->getConfigManager();
    }
    FB_BOOLEAN master_get_process_exiting(IMaster* self)
    {
        return self->getProcessExiting();
    }

    // IStatus
    void status_init(IStatus* self)
    {
        return self->init();
    }
    unsigned status_get_state(const IStatus* self)
    {
        return self->getState();
    }
    void status_set_errors2(IStatus* self, unsigned length, const intptr_t* value)
    {
        return self->setErrors2(length, value);
    }
    void status_set_warnings2(IStatus* self, unsigned length, const intptr_t* value)
    {
        return self->setWarnings2(length, value);
    }
    void status_set_errors(IStatus* self, const intptr_t* value)
    {
        return self->setErrors(value);
    }
    void status_set_warnings(IStatus* self, const intptr_t* value)
    {
        return self->setWarnings(value);
    }
    const intptr_t* status_get_errors(const IStatus* self)
    {
        return self->getErrors();
    }
    const intptr_t* status_get_warnings(const IStatus* self)
    {
        return self->getWarnings();
    }
    IStatus* status_clone(const IStatus* self)
    {
        return self->clone();
    }

    // IProvider
    void provider_shutdown(IProvider* self, StatusWrapper* status, unsigned timeout, const int reason)
    {
        return self->shutdown(status, timeout, reason);
    }
    IAttachment* provider_attach_database(IProvider* self, StatusWrapper* status, const char* fileName, unsigned dpbLength, const unsigned char* dpb)
    {
        return self->attachDatabase(status, fileName, dpbLength, dpb);
    }
    IAttachment* provider_create_database(IProvider* self, StatusWrapper* status, const char* fileName, unsigned dpbLength, const unsigned char* dpb)
    {
        return self->createDatabase(status, fileName, dpbLength, dpb);
    }
    IService* provider_attach_service_manager(IProvider* self, StatusWrapper* status, const char* service, unsigned spbLength, const unsigned char* spb)
    {
        return self->attachServiceManager(status, service, spbLength, spb);
    }
    void provider_set_db_crypt_callback(IProvider* self, StatusWrapper* status, ICryptKeyCallback* cryptCallback)
    {
        return self->setDbCryptCallback(status, cryptCallback);
    }

    // IUtil 
    void util_get_fb_version(IUtil* self, StatusWrapper* status, IAttachment* att, IVersionCallback* callback)
    {
        return self->getFbVersion(status, att, callback);
    }
    void util_load_blob(IUtil* self, StatusWrapper* status, ISC_QUAD* blobId, IAttachment* att, ITransaction* tra, const char* file, FB_BOOLEAN txt)
    {
        return self->loadBlob(status, blobId, att, tra, file, txt);
    }
    void util_dump_blob(IUtil* self, StatusWrapper* status, ISC_QUAD* blobId, IAttachment* att, ITransaction* tra, const char* file, FB_BOOLEAN txt)
    {
        return self->dumpBlob(status, blobId, att, tra, file, txt);
    }
    void util_get_perf_counters(IUtil* self, StatusWrapper* status, IAttachment* att, const char* countersSet, ISC_INT64* counters)
    {
        return self->getPerfCounters(status, att, countersSet, counters);
    }
    IAttachment* util_execute_create_database(IUtil* self, StatusWrapper* status, unsigned stmtLength, const char* creatDBstatement, unsigned dialect, FB_BOOLEAN* stmtIsCreateDb)
    {
        return self->executeCreateDatabase(status, stmtLength, creatDBstatement, dialect, stmtIsCreateDb);
    }
    void util_decode_date(IUtil* self, ISC_DATE date, unsigned* year, unsigned* month, unsigned* day)
    {
        return self->decodeDate(date, year, month, day);
    }
    void util_decode_time(IUtil* self, ISC_TIME time, unsigned* hours, unsigned* minutes, unsigned* seconds, unsigned* fractions)
    {
        return self->decodeTime(time, hours, minutes, seconds, fractions);
    }
    ISC_DATE util_encode_date(IUtil* self, unsigned year, unsigned month, unsigned day)
    {
        return self->encodeDate(year, month, day);
    }
    ISC_TIME util_encode_time(IUtil* self, unsigned hours, unsigned minutes, unsigned seconds, unsigned fractions)
    {
        return self->encodeTime(hours, minutes, seconds, fractions);
    }
    unsigned util_format_status(IUtil* self, char* buffer, unsigned bufferSize, IStatus* status)
    {
        return self->formatStatus(buffer, bufferSize, status);
    }
    unsigned util_get_client_version(IUtil* self)
    {
        return self->getClientVersion();
    }
    IXpbBuilder* util_get_xpb_builder(IUtil* self, StatusWrapper* status, unsigned kind, const unsigned char* buf, unsigned len)
    {
        return self->getXpbBuilder(status, kind, buf, len);
    }
    unsigned util_set_offsets(IUtil* self, StatusWrapper* status, IMessageMetadata* metadata, IOffsetsCallback* callback)
    {
        return self->setOffsets(status, metadata, callback);
    }

    // IXpbBuilder
    void xpb_builder_clear(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->clear(status);
    }
    void xpb_builder_remove_current(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->removeCurrent(status);
    }
    void xpb_builder_insert_int(IXpbBuilder* self, StatusWrapper* status, unsigned char tag, int value)
    {
        return self->insertInt(status, tag, value);
    }
    void xpb_builder_insert_big_int(IXpbBuilder* self, StatusWrapper* status, unsigned char tag, ISC_INT64 value)
    {
        return self->insertBigInt(status, tag, value);
    }
    void xpb_builder_insert_bytes(IXpbBuilder* self, StatusWrapper* status, unsigned char tag, const void* bytes, unsigned length)
    {
        return self->insertBytes(status, tag, bytes, length);
    }
    void xpb_builder_insert_string(IXpbBuilder* self, StatusWrapper* status, unsigned char tag, const char* str)
    {
        return self->insertString(status, tag, str);
    }
    void xpb_builder_insert_tag(IXpbBuilder* self, StatusWrapper* status, unsigned char tag)
    {
        return self->insertTag(status, tag);
    }
    FB_BOOLEAN xpb_builder_is_eof(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->isEof(status);
    }
    void xpb_builder_move_next(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->moveNext(status);
    }
    void xpb_builder_rewind(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->rewind(status);
    }
    FB_BOOLEAN xpb_builder_find_first(IXpbBuilder* self, StatusWrapper* status, unsigned char tag)
    {
        return self->findFirst(status, tag);
    }
    FB_BOOLEAN xpb_builder_find_next(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->findNext(status);
    }
    unsigned char xpb_builder_get_tag(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getTag(status);
    }
    unsigned xpb_builder_get_length(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getLength(status);
    }
    int xpb_builder_get_int(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getInt(status);
    }
    ISC_INT64 xpb_builder_get_big_int(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getBigInt(status);
    }
    const char* xpb_builder_get_string(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getString(status);
    }
    const unsigned char* xpb_builder_get_bytes(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getBytes(status);
    }
    unsigned xpb_builder_get_buffer_length(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getBufferLength(status);
    }
    const unsigned char* xpb_builder_get_buffer(IXpbBuilder* self, StatusWrapper* status)
    {
        return self->getBuffer(status);
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
    void attachment_get_info(IAttachment* self, StatusWrapper* status, unsigned itemsLength, const unsigned char* items, unsigned bufferLength, unsigned char* buffer)
    {
        self->getInfo(status, itemsLength, items, bufferLength, buffer);
    }
    ITransaction* attachment_reconnect_transaction(IAttachment* self, StatusWrapper* status, unsigned length, const unsigned char* id)
    {
        self->reconnectTransaction(status, length, id);
    }
    IRequest* attachment_compile_request(IAttachment* self, StatusWrapper* status, unsigned blrLength, const unsigned char* blr)
    {
        self->compileRequest(status, blrLength, blr);
    }
    void attachment_transact_request(IAttachment* self, StatusWrapper* status, ITransaction* transaction, unsigned blrLength, const unsigned char* blr, unsigned inMsgLength, const unsigned char* inMsg, unsigned outMsgLength, unsigned char* outMsg)
    {
        self->transactRequest(status, transaction, blrLength, blr, inMsgLength, inMsg, outMsgLength, outMsg);
    }
    void attachment_execute_dyn(IAttachment* self, StatusWrapper* status, ITransaction* transaction, unsigned length, const unsigned char* dyn)
    {
        self->executeDyn(status, transaction, length, dyn);
    }
    IResultSet* attachment_open_cursor(IAttachment* self, StatusWrapper* status, ITransaction* transaction, unsigned stmtLength, const char* sqlStmt, unsigned dialect, IMessageMetadata* inMetadata, void* inBuffer, IMessageMetadata* outMetadata, const char* cursorName, unsigned cursorFlags)
    {
        self->openCursor(status, transaction, stmtLength, sqlStmt, dialect, inMetadata, inBuffer, outMetadata, cursorName, cursorFlags);
    }
    IEvents* attachment_que_events(IAttachment* self, StatusWrapper* status, IEventCallback* callback, unsigned length, const unsigned char* events)
    {
        self->queEvents(status, callback, length, events);
    }
    void attachment_cancel_operation(IAttachment* self, StatusWrapper* status, int option)
    {
        self->cancelOperation(status, option);
    }
    void attachment_ping(IAttachment* self, StatusWrapper* status)
    {
        self->ping(status);
    }
    void attachment_drop_database(IAttachment* self, StatusWrapper* status)
    {
        self->dropDatabase(status);
    }

    // ITransaction 
    void transaction_get_info(ITransaction* self, StatusWrapper* status, unsigned itemsLength, const unsigned char* items, unsigned bufferLength, unsigned char* buffer)
    {
        self->getInfo(status, itemsLength, items, bufferLength, buffer);
    }
    void transaction_prepare(ITransaction* self, StatusWrapper* status, unsigned msgLength, const unsigned char* message)
    {
        self->prepare(status, msgLength, message);
    }
    void transaction_commit(ITransaction* self, StatusWrapper* status)
    {
        self->commit(status);
    }
    void transaction_commit_retaining(ITransaction* self, StatusWrapper* status)
    {
        self->commitRetaining(status);
    }
    void transaction_rollback(ITransaction* self, StatusWrapper* status)
    {
        self->rollback(status);
    }
    void transaction_rollback_retaining(ITransaction* self, StatusWrapper* status)
    {
        self->rollbackRetaining(status);
    }
    void transaction_disconnect(ITransaction* self, StatusWrapper* status)
    {
        self->disconnect(status);
    }
    ITransaction* transaction_join(ITransaction* self, StatusWrapper* status, ITransaction* transaction)
    {
        self->join(status, transaction);
    }
    ITransaction* transaction_validate(ITransaction* self, StatusWrapper* status, IAttachment* attachment)
    {
        self->validate(status, attachment);
    }
    ITransaction* transaction_enter_dtc(ITransaction* self, StatusWrapper* status)
    {
        self->enterDtc(status);
    }

    // IStatement
    void statement_get_info(IStatement* self, StatusWrapper* status, unsigned itemsLength, const unsigned char* items, unsigned bufferLength, unsigned char* buffer)
    {
        return self->getInfo(status, itemsLength, items, bufferLength, buffer);
    }
    unsigned statement_get_type(IStatement* self, StatusWrapper* status)
    {
        return self->getType(status);
    }
    const char* statement_get_plan(IStatement* self, StatusWrapper* status, FB_BOOLEAN detailed)
    {
        return self->getPlan(status, detailed);
    }
    ISC_UINT64 statement_get_affected_records(IStatement* self, StatusWrapper* status)
    {
        return self->getAffectedRecords(status);
    }
    IMessageMetadata* statement_get_input_metadata(IStatement* self, StatusWrapper* status)
    {
        return self->getInputMetadata(status);
    }
    IMessageMetadata* statement_get_output_metadata(IStatement* self, StatusWrapper* status)
    {
        return self->getOutputMetadata(status);
    }
    ITransaction* statement_execute(IStatement* self, StatusWrapper* status, ITransaction* transaction, IMessageMetadata* inMetadata, void* inBuffer, IMessageMetadata* outMetadata, void* outBuffer)
    {
        return self->execute(status, transaction, inMetadata, inBuffer, outMetadata, outBuffer);
    }
    IResultSet* statement_open_cursor(IStatement* self, StatusWrapper* status, ITransaction* transaction, IMessageMetadata* inMetadata, void* inBuffer, IMessageMetadata* outMetadata, unsigned flags)
    {
        return self->openCursor(status, transaction, inMetadata, inBuffer, outMetadata, flags);
    }
    void statement_set_cursor_name(IStatement* self, StatusWrapper* status, const char* name)
    {
        return self->setCursorName(status, name);
    }
    void statement_free(IStatement* self, StatusWrapper* status)
    {
        return self->free(status);
    }
    unsigned statement_get_flags(IStatement* self, StatusWrapper* status)
    {
        return self->getFlags(status);
    }

    // IMessageMetadata
    unsigned message_metadata_get_count(IMessageMetadata* self, StatusWrapper* status)
    {
        return self->getCount(status);
    }
    const char* message_metadata_get_field(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getField(status, index);
    }
    const char* message_metadata_get_relation(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getRelation(status, index);
    }
    const char* message_metadata_get_owner(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getOwner(status, index);
    }
    const char* message_metadata_get_alias(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getAlias(status, index);
    }
    unsigned message_metadata_get_type(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getType(status, index);
    }
    FB_BOOLEAN message_metadata_is_nullable(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->isNullable(status, index);
    }
    int message_metadata_get_sub_type(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getSubType(status, index);
    }
    unsigned message_metadata_get_length(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getLength(status, index);
    }
    int message_metadata_get_scale(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getScale(status, index);
    }
    unsigned message_metadata_get_char_set(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getCharSet(status, index);
    }
    unsigned message_metadata_get_offset(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getOffset(status, index);
    }
    unsigned message_metadata_get_null_offset(IMessageMetadata* self, StatusWrapper* status, unsigned index)
    {
        return self->getNullOffset(status, index);
    }
    IMetadataBuilder* message_metadata_get_builder(IMessageMetadata* self, StatusWrapper* status)
    {
        return self->getBuilder(status);
    }
    unsigned message_metadata_get_message_length(IMessageMetadata* self, StatusWrapper* status)
    {
        return self->getMessageLength(status);
    }

    // IMetadataBuilder
    void metadata_builder_set_type(IMetadataBuilder* self, StatusWrapper* status, unsigned index, unsigned type)
    {
        return self->setType(status, index, type);
    }
    void metadata_builder_set_sub_type(IMetadataBuilder* self, StatusWrapper* status, unsigned index, int subType)
    {
        return self->setSubType(status, index, subType);
    }
    void metadata_builder_set_length(IMetadataBuilder* self, StatusWrapper* status, unsigned index, unsigned length)
    {
        return self->setLength(status, index, length);
    }
    void metadata_builder_set_char_set(IMetadataBuilder* self, StatusWrapper* status, unsigned index, unsigned charSet)
    {
        return self->setCharSet(status, index, charSet);
    }
    void metadata_builder_set_scale(IMetadataBuilder* self, StatusWrapper* status, unsigned index, int scale)
    {
        return self->setScale(status, index, scale);
    }
    void metadata_builder_truncate(IMetadataBuilder* self, StatusWrapper* status, unsigned count)
    {
        return self->truncate(status, count);
    }
    void metadata_builder_move_name_to_index(IMetadataBuilder* self, StatusWrapper* status, const char* name, unsigned index)
    {
        return self->moveNameToIndex(status, name, index);
    }
    void metadata_builder_remove(IMetadataBuilder* self, StatusWrapper* status, unsigned index)
    {
        return self->remove(status, index);
    }
    unsigned metadata_builder_add_field(IMetadataBuilder* self, StatusWrapper* status)
    {
        return self->addField(status);
    }
    IMessageMetadata* metadata_builder_get_metadata(IMetadataBuilder* self, StatusWrapper* status)
    {
        return self->getMetadata(status);
    }

    // IResultSet
    int result_set_fetch_next(IResultSet* self, StatusWrapper* status, void* message)
    {
        return self->fetchNext(status, message);
    }
    int result_set_fetch_prior(IResultSet* self, StatusWrapper* status, void* message)
    {
        return self->fetchPrior(status, message);
    }
    int result_set_fetch_first(IResultSet* self, StatusWrapper* status, void* message)
    {
        return self->fetchFirst(status, message);
    }
    int result_set_fetch_last(IResultSet* self, StatusWrapper* status, void* message)
    {
        return self->fetchLast(status, message);
    }
    int result_set_fetch_absolute(IResultSet* self, StatusWrapper* status, int position, void* message)
    {
        return self->fetchAbsolute(status, position, message);
    }
    int result_set_fetch_relative(IResultSet* self, StatusWrapper* status, int offset, void* message)
    {
        return self->fetchRelative(status, offset, message);
    }
    FB_BOOLEAN result_set_is_eof(IResultSet* self, StatusWrapper* status)
    {
        return self->isEof(status);
    }
    FB_BOOLEAN result_set_is_bof(IResultSet* self, StatusWrapper* status)
    {
        return self->isBof(status);
    }
    IMessageMetadata* result_set_get_metadata(IResultSet* self, StatusWrapper* status)
    {
        return self->getMetadata(status);
    }
    void result_set_close(IResultSet* self, StatusWrapper* status)
    {
        return self->close(status);
    }
    void result_set_set_delayed_output_format(IResultSet* self, StatusWrapper* status, IMessageMetadata* format)
    {
        return self->setDelayedOutputFormat(status, format);
    }
}