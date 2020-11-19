use crate::requests;
use azure_core::No;

use super::*;

#[derive(Debug, Clone)]
pub struct AttachmentClient {
    document_client: DocumentClient,
    attachment_name: String,
}

impl AttachmentClient {
    pub(crate) fn new(document_client: DocumentClient, attachment_name: String) -> Self {
        Self {
            document_client,
            attachment_name,
        }
    }

    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.document_client().hyper_client()
    }

    fn cosmos_client(&self) -> &CosmosClient {
        self.document_client().cosmos_client()
    }

    fn database_client(&self) -> &DatabaseClient {
        self.document_client().database_client()
    }

    fn collection_client(&self) -> &CollectionClient {
        self.document_client().collection_client()
    }

    fn document_client(&self) -> &DocumentClient {
        &self.document_client
    }

    fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_, No, No> {
        requests::CreateSlugAttachmentBuilder::new(self)
    }

    fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, No, No> {
        requests::ReplaceSlugAttachmentBuilder::new(self)
    }

    fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_, No, No> {
        requests::CreateReferenceAttachmentBuilder::new(self)
    }

    fn replace_reference(&self) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, No, No> {
        requests::ReplaceReferenceAttachmentBuilder::new(self)
    }

    fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_> {
        requests::DeleteAttachmentBuilder::new(self)
    }

    fn get(&self) -> requests::GetAttachmentBuilder<'_, '_> {
        requests::GetAttachmentBuilder::new(self)
    }
}
