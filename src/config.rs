use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    pub build: Option<Vec<Build>>,
    pub deploy: Option<Vec<Step>>,
    pub storage: Option<Vec<BTreeMap<String, String>>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Step {
    pub name: String,
    #[serde(rename = "do")]
    pub make: Option<String>,
    pub put: Option<String>,
    pub get: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Build {
    Make(String),
    Step(Step),
}
