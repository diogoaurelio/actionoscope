use crate::models::github_workflows::Step;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub description: Option<String>,
    pub runs: Vec<ActionBody>,
    pub inputs: Option<collections::HashMap<String, ActionVariableInput>>,
}

impl Action {
    pub fn from_yaml(yaml_data: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml_data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionVariableInput {
    pub description: Option<String>,
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionBody {
    // there are numerous options for using, so many that not considering using an enum;
    // here are some examples: composite / docker / node16 / ...
    pub using: String,
    pub image: Option<String>,
    pub main: Option<String>,
    // steps are used only for composite actions
    pub steps: Option<Vec<Step>>,
}
