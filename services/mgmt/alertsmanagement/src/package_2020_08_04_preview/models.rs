#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertAction {
    #[serde(rename = "actionGroupId", skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[serde(rename = "webHookProperties", skip_serializing_if = "Option::is_none")]
    pub web_hook_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertProperties {
    pub description: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    pub criteria: HealthAlertCriteria,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<HealthAlertAction>,
    #[serde(rename = "lastUpdatedTime", skip_serializing)]
    pub last_updated_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: HealthAlertProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertResourcePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthAlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertResourceCollection {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthAlertResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertCriteria {
    #[serde(rename = "allOf", skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<HealthAlertCriterion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertCriterion {
    pub namespace: health_alert_criterion::Namespace,
}
pub mod health_alert_criterion {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Namespace {
        GuestVmHealth,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmGuestHealthAlertCriterion {
    #[serde(flatten)]
    pub health_alert_criterion: HealthAlertCriterion,
    #[serde(rename = "monitorNames", skip_serializing_if = "Vec::is_empty")]
    pub monitor_names: Vec<String>,
    #[serde(rename = "monitorTypes", skip_serializing_if = "Vec::is_empty")]
    pub monitor_types: Vec<String>,
    #[serde(rename = "healthStates")]
    pub health_states: Vec<HealthState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthState {
    #[serde(rename = "healthStateName")]
    pub health_state_name: health_state::HealthStateName,
    pub severity: i64,
}
pub mod health_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HealthStateName {
        Warning,
        Critical,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertsErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HealthAlertsErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthAlertsErrorResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}