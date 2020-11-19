use crate::requests;
use super::*;

#[derive(Debug, Clone)]
pub struct PermissionClient
{
    user_client: UserClient;
    permission_name: String,
}

impl PermissionStruct {
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

    fn create_permission(&self) -> requests::CreatePermissionBuilder<'_, '_, C, D, USER> {
        requests::CreatePermissionBuilder::new(self)
    }

    fn replace_permission(&self) -> requests::ReplacePermissionBuilder<'_, '_, C, D, USER> {
        requests::ReplacePermissionBuilder::new(self)
    }

    fn get_permission(&self) -> requests::GetPermissionBuilder<'_, '_, C, D, USER> {
        requests::GetPermissionBuilder::new(self)
    }

    fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, '_, C, D, USER> {
        requests::DeletePermissionsBuilder::new(self)
    }
}
