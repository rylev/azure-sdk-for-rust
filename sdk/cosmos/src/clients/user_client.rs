use super::*;
use crate::requests;
use crate::ResourceType;
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

    pub fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.database_client().hyper_client()
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        self.database_client().cosmos_client()
    }

    pub fn database_client(&self) -> &DatabaseClient {
        &self.database_client
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn create_user(&self) -> requests::CreateUserBuilder<'_, '_> {
        requests::CreateUserBuilder::new(self)
    }

    pub fn get_user(&self) -> requests::GetUserBuilder<'_, '_> {
        requests::GetUserBuilder::new(self)
    }

    pub fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, '_, No> {
        requests::ReplaceUserBuilder::new(self)
    }

    pub fn delete_user(&self) -> requests::DeleteUserBuilder<'_, '_> {
        requests::DeleteUserBuilder::new(self)
    }

    pub fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, '_> {
        requests::ListPermissionsBuilder::new(self)
    }

    pub fn into_permission_client(self, permission_name: String) -> PermissionClient {
        PermissionClient::new(self, permission_name)
    }

    pub fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}/users", self.database_client().database_name()),
            method,
            ResourceType::Users,
        )
    }

    pub fn prepare_request_with_user_name(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/users/{}",
                self.database_client().database_name(),
                self.user_name()
            ),
            method,
            ResourceType::Users,
        )
    }
}
