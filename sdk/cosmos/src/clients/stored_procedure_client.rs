use super::CollectionClient;
use crate::requests;
use azure_core::No;

#[derive(Debug, Clone)]
pub struct StoredProcedureClient {
    collection_client: CollectionClient,
    stored_procedure_name: String,
}

impl StoredProcedureClient {
    pub(crate) fn new(collection_client: CollectionClient, stored_procedure_name: String) -> Self {
        Self {
            collection_client,
            stored_procedure_name,
        }
    }

    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client.hyper_client()
    }

    fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client.cosmos_client()
    }

    fn database_client(&self) -> &DatabaseClient {
        self.collection_client.database_client()
    }

    fn collection_client(&self) -> &COLL {
        &self.collection_client
    }

    fn stored_procedure_name(&self) -> &str {
        &self.stored_procedure_name
    }

    fn create_stored_procedure(
        &self,
    ) -> requests::CreateStoredProcedureBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateStoredProcedureBuilder::new(self)
    }

    fn replace_stored_procedure(
        &self,
    ) -> requests::ReplaceStoredProcedureBuilder<'_, '_, C, D, COLL, No> {
        requests::ReplaceStoredProcedureBuilder::new(self)
    }

    fn execute_stored_procedure(
        &self,
    ) -> requests::ExecuteStoredProcedureBuilder<'_, '_, C, D, COLL> {
        requests::ExecuteStoredProcedureBuilder::new(self)
    }

    fn delete_stored_procedure(
        &self,
    ) -> requests::DeleteStoredProcedureBuilder<'_, '_, C, D, COLL> {
        requests::DeleteStoredProcedureBuilder::new(self)
    }
}
