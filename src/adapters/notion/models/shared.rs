use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    pub link: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text,
    pub annotations: Option<Annotations>,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText {
    #[serde(rename = "rich_text")]
    pub type_field: String,
    pub text: Text,
}

/// Parent enum
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Parent {
    Page(PageParent),
    Database(DatabaseParent),
    #[default]
    None,
}

/// Page parent struct
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "page_id")]
    pub page_id: String,
}

/// Database parent struct
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseParent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "database_id")]
    pub database_id: String,
}
