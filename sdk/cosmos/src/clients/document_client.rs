use super::{AttachmentClient, CollectionClient, CosmosClient, DatabaseClient};
use crate::requests;
use crate::PartitionKeys;

#[derive(Debug, Clone)]
pub struct DocumentClient {
    collection_client: CollectionClient,
    document_name: String,
    partition_keys: PartitionKeys,
}

impl DocumentClient {
    pub(crate) fn new(
        collection_client: CollectionClient,
        document_name: String,
        partition_keys: PartitionKeys,
    ) -> Self {
        Self {
            collection_client,
            document_name,
            partition_keys,
        }
    }

    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client().hyper_client()
    }

    fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client().cosmos_client()
    }

    fn database_client(&self) -> &DatabaseClient {
        self.collection_client().database_client()
    }

    fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    fn document_name(&self) -> &str {
        &self.document_name
    }

    fn partition_keys(&self) -> &PartitionKeys {
        &self.partition_keys
    }

    fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_> {
        requests::GetDocumentBuilder::new(self)
    }

    fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_> {
        requests::DeleteDocumentBuilder::new(self)
    }

    fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_> {
        requests::ListAttachmentsBuilder::new(self)
    }

    fn into_attachment_client(self, attachment_name: String) -> AttachmentClient {
        AttachmentClient::new(self, attachment_name)
    }
}
