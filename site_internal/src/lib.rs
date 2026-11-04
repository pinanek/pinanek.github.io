use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub rev: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuerySource {
    pub repo: String,
    pub path: String,
    #[serde(rename = "crate")]
    pub grammar_crate: Option<String>,
    pub language: Option<String>,
    pub variant: Option<String>,
    pub display_name: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
}
