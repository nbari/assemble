use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub env: BTreeMap<String, String>,
    pub steps: Vec<Steps>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Steps {
    pub name: String,
    pub cmd: String,
}
