use super::*;
use crate::requests;
use azure_core::No;

#[derive(Debug, Clone)]
pub struct UserDefinedFunctionClient {
    collection_client: CollectionClient,
    user_defined_function_name: String,
}

impl UserDefinedFunctionClient {
    pub(crate) fn new(
        collection_client: CollectionClient,
        user_defined_function_name: String,
    ) -> Self {
        Self {
            collection_client,
            user_defined_function_name,
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

    fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    fn user_defined_function_name(&self) -> &str {
        &self.user_defined_function_name
    }

    fn create_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, true)
    }

    fn replace_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, false)
    }

    fn delete_user_defined_function(
        &self,
    ) -> requests::DeleteUserDefinedFunctionBuilder<'_, '_, C, D, COLL> {
        requests::DeleteUserDefinedFunctionBuilder::new(self)
    }
}
