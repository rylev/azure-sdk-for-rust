use crate::prelude::*;
use crate::responses::ListStoredProceduresResponse;
use crate::ResourceType;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use futures::stream::{unfold, Stream};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListStoredProceduresBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
    continuation: Option<&'b str>,
    max_item_count: i32,
}

impl<'a, 'b> ListStoredProceduresBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
        }
    }
}

impl<'a, 'b> CollectionClientRequired<'a> for ListStoredProceduresBuilder<'a, 'b> {
    fn collection_client(&self) -> &'a CollectionClient {
        self.collection_client
    }
}

impl<'a, 'b> UserAgentOption<'b> for ListStoredProceduresBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for ListStoredProceduresBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for ListStoredProceduresBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> ContinuationOption<'b> for ListStoredProceduresBuilder<'a, 'b> {
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b> MaxItemCountOption for ListStoredProceduresBuilder<'a, 'b> {
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b> UserAgentSupport<'b> for ListStoredProceduresBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for ListStoredProceduresBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for ListStoredProceduresBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a, 'b> ContinuationSupport<'b> for ListStoredProceduresBuilder<'a, 'b> {
    type O = Self;

    fn with_continuation(self, continuation: &'b str) -> Self::O {
        Self {
            continuation: Some(continuation),
            ..self
        }
    }
}

impl<'a, 'b> MaxItemCountSupport for ListStoredProceduresBuilder<'a, 'b> {
    type O = Self;

    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        Self {
            max_item_count,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ListStoredProceduresBuilder<'a, 'b> {
    pub async fn execute(&self) -> Result<ListStoredProceduresResponse, AzureError> {
        trace!("ListStoredProceduresBuilder::execute called");

        let request = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/sprocs",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name(),
            ),
            hyper::Method::GET,
            ResourceType::StoredProcedures,
        );

        // add trait headers
        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);
        let request = ContinuationOption::add_header(self, request);
        let request = MaxItemCountOption::add_header(self, request);

        let request = request.body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client().hyper_client().request(request),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }

    pub fn stream(
        &self,
    ) -> impl Stream<Item = Result<ListStoredProceduresResponse, AzureError>> + '_ {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            Continuation(String),
        };

        unfold(
            Some(States::Init),
            move |continuation_token: Option<States>| {
                async move {
                    debug!("continuation_token == {:?}", &continuation_token);
                    let response = match continuation_token {
                        Some(States::Init) => self.execute().await,
                        Some(States::Continuation(continuation_token)) => {
                            self.clone()
                                .with_continuation(&continuation_token)
                                .execute()
                                .await
                        }
                        None => return None,
                    };

                    // the ? operator does not work in async move (yet?)
                    // so we have to resort to this boilerplate
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => return Some((Err(err), None)),
                    };

                    let continuation_token = match &response.continuation_token {
                        Some(ct) => Some(States::Continuation(ct.to_owned())),
                        None => None,
                    };

                    Some((Ok(response), continuation_token))
                }
            },
        )
    }
}
