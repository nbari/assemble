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
    pub storage: Option<S3>,
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct S3 {
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub access_key: String,
    #[serde(default)]
    pub secret_key: String,
    pub bucket: String,
}
