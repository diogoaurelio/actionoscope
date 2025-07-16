use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize)]
pub struct Push {
    pub branches: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trigger {
    pub push: Option<Push>,
    pub pull_request: Option<serde_yaml::Value>, // Using Value for unstructured data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub id: Option<String>,
    pub uses: Option<String>,
    pub shell: Option<String>,
    #[serde(rename = "working-directory")]
    pub working_directory: Option<String>,
    pub run: Option<String>,
}

impl Step {
    pub fn get_name_or_id(&self) -> &str {
        self.name
            .as_deref()
            .unwrap_or(self.id.as_deref().unwrap_or("unknown"))
    }

    pub fn get_id(&self) -> &str {
        self.id.as_deref().unwrap_or("unknown")
    }

    pub fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or("unknown")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub on: Trigger,
    pub jobs: collections::HashMap<String, Job>,
    pub env: Option<collections::HashMap<String, String>>,
}

impl Workflow {
    pub fn get_job(&self, job_name: &str) -> Option<&Job> {
        self.jobs.get(job_name)
    }

    pub fn from_yaml(yaml_data: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml_data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunsOn {
    Single(String),
    Multiple(Vec<String>),
}

impl PartialEq<String> for RunsOn {
    fn eq(&self, other: &String) -> bool {
        match self {
            RunsOn::Single(s) => s == other,
            RunsOn::Multiple(v) => v.contains(other),
        }
    }
}

impl PartialEq<RunsOn> for String {
    fn eq(&self, other: &RunsOn) -> bool {
        other == self
    }
}

impl PartialEq<Vec<String>> for RunsOn {
    fn eq(&self, other: &Vec<String>) -> bool {
        match self {
            RunsOn::Single(s) => other.len() == 1 && other[0] == *s,
            RunsOn::Multiple(v) => v == other,
        }
    }
}

impl PartialEq<RunsOn> for Vec<String> {
    fn eq(&self, other: &RunsOn) -> bool {
        match other {
            RunsOn::Single(s) => self.len() == 1 && self[0] == *s,
            RunsOn::Multiple(v) => self == v,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "runs-on")]
    pub runs_on: RunsOn,
    pub steps: Vec<Step>,
}

impl Job {
    pub fn get_step(&self, id_or_name: &str) -> Option<&Step> {
        self.steps.iter().find(|step| {
            step.name.as_deref() == Some(id_or_name) || step.id.as_deref() == Some(id_or_name)
        })
    }
    pub fn get_all_steps_since(
        &self,
        start_step_id_or_name: Option<&str>,
        end_step_id_or_name: Option<&str>,
    ) -> Vec<&Step> {
        let mut steps = Vec::new();
        let mut found = false;
        for step in &self.steps {
            if start_step_id_or_name.is_none()
                || step.name.as_deref() == start_step_id_or_name
                || step.id.as_deref() == start_step_id_or_name
            {
                found = true;
            }

            if found {
                steps.push(step);
            }
            if end_step_id_or_name.is_some()
                && (step.name.as_deref() == end_step_id_or_name
                    || step.id.as_deref() == end_step_id_or_name)
            {
                break;
            }
        }
        steps
    }
}
