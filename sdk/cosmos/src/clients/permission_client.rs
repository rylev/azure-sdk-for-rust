use super::*;
use crate::requests;

#[derive(Debug, Clone)]
pub struct PermissionClient {
    user_client: UserClient,
    permission_name: String,
}

impl PermissionClient {
    pub(crate) fn new(user_client: UserClient, permission_name: String) -> Self {
        Self {
            user_client,
            permission_name,
        }
    }

    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.user_client.hyper_client()
    }

    fn cosmos_client(&self) -> &CosmosClient {
        self.user_client.cosmos_client()
    }

    fn database_client(&self) -> &DatabaseClient {
        self.user_client.database_client()
    }

    fn user_client(&self) -> &UserClient {
        &self.user_client
    }

    fn permission_name(&self) -> &str {
        &self.permission_name
    }

    fn create_permission(&self) -> requests::CreatePermissionBuilder<'_, '_> {
        requests::CreatePermissionBuilder::new(self)
    }

    fn replace_permission(&self) -> requests::ReplacePermissionBuilder<'_, '_> {
        requests::ReplacePermissionBuilder::new(self)
    }

    fn get_permission(&self) -> requests::GetPermissionBuilder<'_, '_> {
        requests::GetPermissionBuilder::new(self)
    }

    fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, '_> {
        requests::DeletePermissionsBuilder::new(self)
    }
}
