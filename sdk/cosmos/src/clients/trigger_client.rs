use super::*;
use crate::requests;
use azure_core::No;

#[derive(Debug, Clone)]
pub struct TriggerClient {
    collection_client: CollectionClient,
    trigger_name: String,
}

impl TriggerClient {
    pub(crate) fn new(collection_client: CollectionClient, trigger_name: String) -> Self {
        Self {
            collection_client,
            trigger_name,
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

    fn trigger_name(&self) -> &str {
        &self.trigger_name
    }

    fn create_trigger(
        &self,
    ) -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, true)
    }

    fn replace_trigger(
        &self,
    ) -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, false)
    }

    fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, '_, C, D, COLL> {
        requests::DeleteTriggerBuilder::new(self)
    }
}
