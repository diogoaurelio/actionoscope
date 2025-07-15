use crate::models::github_workflows::Job;
use std::collections::HashMap;

pub struct RunJobConfig<'a> {
    pub jobs: Vec<&'a Job>,
    pub job_names: Vec<String>,
    pub step: Option<String>,
    pub from_step: Option<String>,
    pub to_step: Option<String>,
    pub env_vars: Option<HashMap<String, String>>,
    pub secret_vars: Option<HashMap<String, String>>,
    pub steps_to_skip: Vec<String>,
}
