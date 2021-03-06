use crate::core::prelude::*;
use crate::queue::clients::QueueClient;
use crate::queue::HasStorageClient;
use crate::responses::*;
use azure_core::errors::AzureError;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;
use url::Url;

#[derive(Debug)]
pub struct ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_client: &'a QueueClient<C>,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a, C> ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_client: &'a QueueClient<C>) -> Self {
        ClearMessagesBuilder {
            queue_client,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, C> ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(self) -> Result<ClearMessagesResponse, AzureError> {
        let mut url = Url::parse(&format!(
            "{}/{}/messages",
            self.queue_client.storage_client().queue_uri(),
            self.queue_client.queue_name(),
        ))?;

        AppendToUrlQuery::append_to_url_query(&self.timeout, &mut url);
        debug!("url == {}", url);

        let perform_request_response = self.queue_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::NO_CONTENT)
            .await?;

        (&headers).try_into()
    }
}
