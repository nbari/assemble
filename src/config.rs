use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    pub build: Option<Vec<Steps>>,
    pub deploy: Option<Vec<Steps>>,
    pub storage: Option<Vec<BTreeMap<String, String>>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Steps {
    pub name: String,
    pub r#do: Option<String>,
    pub put: Option<String>,
    pub get: Option<String>,
}
