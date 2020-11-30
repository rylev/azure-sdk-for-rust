//! Utilities for interacting with [`User`]s.

use super::Resource;

/// A logical namespace for scoping permissions on resources.
///
/// You can learn more about users [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/users).
#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
pub struct User {
    pub id: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(skip_serializing)]
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(skip_serializing)]
    #[serde(rename = "_permissions")]
    pub permissions: String,
}

impl std::convert::TryFrom<&[u8]> for User {
    type Error = serde_json::Error;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(body)
    }
}

impl Resource for User {
    fn uri(&self) -> &str {
        &self._self
    }
}

impl Resource for &User {
    fn uri(&self) -> &str {
        &self._self
    }
}
