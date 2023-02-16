use serde::{Deserialize, Serialize};

use super::shared::Title;

// Name property with Vec<Title>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: Vec<Title>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "Name")]
    pub name: Name,
}
