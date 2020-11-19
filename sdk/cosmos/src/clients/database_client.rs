use super::*;
use crate::requests;
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

    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }

    fn cosmos_client(&self) -> &CosmosClient {
        &self.cosmos_client
    }

    fn database_name(&self) -> &str {
        &self.database_name
    }

    fn list_collections(&self) -> requests::ListCollectionsBuilder<'_> {
        requests::ListCollectionsBuilder::new(self)
    }

    fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_> {
        requests::GetDatabaseBuilder::new(self)
    }

    fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_> {
        requests::DeleteDatabaseBuilder::new(self)
    }

    fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, No, No, No, No> {
        requests::CreateCollectionBuilder::new(self)
    }

    fn list_users(&self) -> requests::ListUsersBuilder<'_, '_> {
        requests::ListUsersBuilder::new(self)
    }

    fn into_collection_client<IntoCowStr>(self, collection_name: String) -> CollectionClient {
        CollectionClient::new(self, collection_name)
    }

    fn into_user_client<IntoCowStr>(self, user_name: IntoCowStr) -> UserClient {
        UserClient::new(self, user_name)
    }
}
