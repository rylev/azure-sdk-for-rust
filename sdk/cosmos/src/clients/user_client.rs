use super::*;
use crate::requests;
use azure_core::No;

#[derive(Debug, Clone)]
pub struct UserClient {
    database_client: DatabaseClient,
    user_name: String,
}

impl UserClient {
    pub(crate) fn new(database_client: DatabaseClient, user_name: String) -> Self {
        Self {
            database_client,
            user_name,
        }
    }

    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.database_client().hyper_client()
    }

    fn cosmos_client(&self) -> &CosmosClient {
        self.database_client().cosmos_client()
    }

    fn database_client(&self) -> &DatabaseClient {
        &self.database_client
    }

    fn user_name(&self) -> &str {
        &self.user_name
    }

    fn create_user(&self) -> requests::CreateUserBuilder<'_, '_> {
        requests::CreateUserBuilder::new(self)
    }

    fn get_user(&self) -> requests::GetUserBuilder<'_, '_> {
        requests::GetUserBuilder::new(self)
    }

    fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, '_, No> {
        requests::ReplaceUserBuilder::new(self)
    }

    fn delete_user(&self) -> requests::DeleteUserBuilder<'_, '_> {
        requests::DeleteUserBuilder::new(self)
    }

    fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, '_> {
        requests::ListPermissionsBuilder::new(self)
    }

    fn into_permission_client(self, permission_name: String) -> PermissionClient {
        PermissionClient::new(self, permission_name)
    }
}
