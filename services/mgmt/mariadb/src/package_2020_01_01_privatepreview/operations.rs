#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod server_keys {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list_by_instance(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<ServerKeyListResult, list_by_instance::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DBforMariaDB/servers/{}/keys",
            &operation_config.base_path, subscription_id, resource_group_name, server_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_by_instance::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_by_instance::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_instance::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_instance::ResponseBytesError)?;
                let rsp_value: ServerKeyListResult = serde_json::from_slice(&body).context(list_by_instance::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_instance::ResponseBytesError)?;
                let rsp_value: CloudError = serde_json::from_slice(&body).context(list_by_instance::DeserializeError { body })?;
                list_by_instance::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod list_by_instance {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::CloudError,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        key_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<ServerKey, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DBforMariaDB/servers/{}/keys/{}",
            &operation_config.base_path, subscription_id, resource_group_name, server_name, key_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: ServerKey = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: CloudError = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                get::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::CloudError,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        key_name: &str,
        parameters: &ServerKey,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DBforMariaDB/servers/{}/keys/{}",
            &operation_config.base_path, subscription_id, resource_group_name, server_name, key_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(create_or_update::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: ServerKey = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => Ok(create_or_update::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: CloudError = serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                create_or_update::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(ServerKey),
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::CloudError,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        server_name: &str,
        key_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DBforMariaDB/servers/{}/keys/{}",
            &operation_config.base_path, subscription_id, resource_group_name, server_name, key_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(delete::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                let rsp_value: CloudError = serde_json::from_slice(&body).context(delete::DeserializeError { body })?;
                delete::DefaultResponse {
                    status_code,
                    value: rsp_value,
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            DefaultResponse {
                status_code: StatusCode,
                value: models::CloudError,
            },
            BuildRequestError {
                source: reqwest::Error,
            },
            ExecuteRequestError {
                source: reqwest::Error,
            },
            ResponseBytesError {
                source: reqwest::Error,
            },
            DeserializeError {
                source: serde_json::Error,
                body: bytes::Bytes,
            },
            GetTokenError {
                source: azure_core::errors::AzureError,
            },
        }
    }
}
