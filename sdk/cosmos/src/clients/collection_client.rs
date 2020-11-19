use super::DatabaseClient;
use crate::clients::*;
use crate::requests;
use crate::{PartitionKeys, UserDefinedFunctionClient};
use azure_core::No;

#[derive(Debug, Clone)]
pub struct CollectionClient {
    database_client: DatabaseClient,
    collection_name: String,
}

impl CollectionClient {
    pub(crate) fn new(database_client: DatabaseClient, collection_name: String) -> Self {
        Self {
            database_client,
            collection_name,
        }
    }

    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }

    fn cosmos_client(&self) -> &CosmosClient {
        self.database_client.cosmos_client()
    }

    fn database_client(&self) -> &DatabaseClient {
        &self.database_client
    }

    fn collection_name(&self) -> &str {
        &self.collection_name
    }

    fn get_collection(&self) -> requests::GetCollectionBuilder<'_, C, D> {
        requests::GetCollectionBuilder::new(self)
    }

    fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, C, D> {
        requests::DeleteCollectionBuilder::new(self)
    }

    fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, '_, C, D, No, No> {
        requests::ReplaceCollectionBuilder::new(self)
    }

    fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, C, D> {
        requests::ListDocumentsBuilder::new(self)
    }

    fn create_document(&self) -> requests::CreateDocumentBuilder<'_, '_, C, D, No> {
        requests::CreateDocumentBuilder::new(self)
    }

    fn replace_document(&self) -> requests::ReplaceDocumentBuilder<'_, '_, C, D, No, No> {
        requests::ReplaceDocumentBuilder::new(self)
    }

    fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, C, D, No> {
        requests::QueryDocumentsBuilder::new(self)
    }

    fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, '_, C, D> {
        requests::ListStoredProceduresBuilder::new(self)
    }

    fn list_user_defined_functions(
        &self,
    ) -> requests::ListUserDefinedFunctionsBuilder<'_, '_, C, D> {
        requests::ListUserDefinedFunctionsBuilder::new(self)
    }

    fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, C, D> {
        requests::ListTriggersBuilder::new(self)
    }

    fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, C, D> {
        requests::GetPartitionKeyRangesBuilder::new(self)
    }

    fn into_document_client<DocName>(
        self,
        document_name: String,
        partition_keys: PartitionKeys,
    ) -> DocumentClient {
        DocumentClient::new(self, document_name, partition_keys)
    }

    fn into_trigger_client<IntoCowStr>(
        self,
        trigger_name: String,
    ) -> TriggerStruct<'a, C, D, Self> {
        TriggerStruct::new(Cow::Owned(self), trigger_name.into())
    }

    fn into_user_defined_function_client(
        self,
        user_defined_function_name: String,
    ) -> UserDefinedFunctionStruct<'a, C, D, Self> {
        UserDefinedFunctionStruct::new(Cow::Owned(self), user_defined_function_name.into())
    }

    fn into_stored_procedure_client(self, stored_procedure_name: String) -> StoredProcedureClient {
        StoredProcedureClient::new(self, stored_procedure_name)
    }
}
