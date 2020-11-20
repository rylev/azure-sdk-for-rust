use super::*;
use crate::requests;
use crate::ResourceType;
use azure_core::No;

#[derive(Debug, Clone)]
pub struct DatabaseClient {
    cosmos_client: CosmosClient,
    database_name: String,
}

impl DatabaseClient {
    pub(crate) fn new(cosmos_client: CosmosClient, database_name: String) -> Self {
        Self {
            cosmos_client,
            database_name,
        }
    }

    pub fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        &self.cosmos_client
    }

    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    pub fn list_collections(&self) -> requests::ListCollectionsBuilder<'_> {
        requests::ListCollectionsBuilder::new(self)
    }

    pub fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_> {
        requests::GetDatabaseBuilder::new(self)
    }

    pub fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_> {
        requests::DeleteDatabaseBuilder::new(self)
    }

    pub fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, No, No, No, No> {
        requests::CreateCollectionBuilder::new(self)
    }

    pub fn list_users(&self) -> requests::ListUsersBuilder<'_, '_> {
        requests::ListUsersBuilder::new(self)
    }

    pub fn into_collection_client<IntoCowStr>(self, collection_name: String) -> CollectionClient {
        CollectionClient::new(self, collection_name)
    }

    pub fn into_user_client(self, user_name: String) -> UserClient {
        UserClient::new(self, user_name)
    }

    pub fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client()
            .prepare_request("dbs", method, ResourceType::Databases)
    }

    pub fn prepare_request_with_database_name(
        &self,
        method: hyper::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}", self.database_name()),
            method,
            ResourceType::Databases,
        )
    }
}
