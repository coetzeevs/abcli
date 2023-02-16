use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    properties::Properties,
    shared::{Children, Parent},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub parent: Parent,
    pub properties: Properties,
    pub children: Vec<Children>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub object: String,
    pub id: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "last_edited_time")]
    pub last_edited_time: String,
    #[serde(rename = "created_by")]
    pub created_by: CreatedBy,
    #[serde(rename = "last_edited_by")]
    pub last_edited_by: LastEditedBy,
    pub cover: Value,
    pub icon: Value,
    pub parent: Parent,
    pub archived: bool,
    pub properties: Properties,
    pub url: String,
}

// Leaving it here for now, only visible on response. Can move it if there's a better use case for it
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub object: String,
    pub id: String,
}

// Leaving it here for now, only visible on response. Can move it if there's a better use case for it
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEditedBy {
    pub object: String,
    pub id: String,
}
