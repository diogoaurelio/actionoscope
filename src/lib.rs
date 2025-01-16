use log::{error, info};
use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::thread;

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub on: Trigger,
    pub jobs: std::collections::HashMap<String, Job>,
    pub env: Option<std::collections::HashMap<String, String>>,
}

impl Workflow {
    pub fn get_job(&self, job_name: &str) -> Option<&Job> {
        self.jobs.get(job_name)
    }

    pub fn from_yaml(yaml_data: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml_data)
    }
}

#[derive(Debug, Deserialize)]
pub struct Trigger {
    pub push: Option<Push>,
    pub pull_request: Option<serde_yaml::Value>, // Using Value for unstructured data
}

#[derive(Debug, Deserialize)]
pub struct Push {
    pub branches: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    #[serde(rename = "runs-on")]
    pub runs_on: String,
    pub steps: Vec<Step>,
}

impl Job {
    pub fn get_step(&self, id_or_name: &str) -> Option<&Step> {
        self.steps.iter().find(|step| {
            step.name.as_deref() == Some(id_or_name) || step.id.as_deref() == Some(id_or_name)
        })
    }
    pub fn get_all_steps_since(&self, id_or_name: &str) -> Vec<&Step> {
        let mut steps = Vec::new();
        let mut found = false;
        for step in &self.steps {
            if step.name.as_deref() == Some(id_or_name) || step.id.as_deref() == Some(id_or_name) {
                found = true;
            }

            if found {
                steps.push(step);
            }
        }
        steps
    }
}

#[derive(Debug, Deserialize)]
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

    fn replace_env_vars(
        command: &str,
        env_vars: Option<std::collections::HashMap<String, String>>,
    ) -> String {
        if env_vars.is_none() {
            return command.to_string();
        }
        let env_vars = env_vars.unwrap().clone();
        let re = regex::Regex::new(r"\$\{\{\s*env\.(\w+)\s*\}\}").unwrap();
        re.replace_all(command, |caps: &regex::Captures| {
            env_vars
                .get(&caps[1])
                .cloned()
                .or_else(|| std::env::var(&caps[1]).ok())
                .unwrap_or_else(|| "".to_string())
        })
        .to_string()
    }

    pub fn run_cmd(
        &self,
        env_vars: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let step_id = self.get_name_or_id();
        if self.run.is_none() {
            let err = format!("No run command found for step id/name '{step_id}'");
            error!(
                "{}; Step details are:\nname: {}\nid: {}\nuses: {}\nshell: {}",
                err,
                self.name.as_deref().unwrap_or("NA"),
                self.id.as_deref().unwrap_or("NA"),
                self.uses.as_deref().unwrap_or("NA"),
                self.shell.as_deref().unwrap_or("NA")
            );
            return Err(err.into());
        }

        let command = self.run.as_deref().unwrap();
        let command = Self::replace_env_vars(command, env_vars).trim().to_string();

        let shell = self.shell.as_deref().unwrap_or("bash");
        let original_dir = std::env::current_dir()?;

        if self.working_directory.is_some() {
            info!(
                "Changing working directory to: {}/{}",
                original_dir.display(),
                self.working_directory.as_deref().unwrap()
            );
            std::env::set_current_dir(self.working_directory.as_deref().unwrap())?;
        }

        info!("Running step name/id '{step_id}', using {shell} shell, with command: \n{command}\n");

        let mut child = Command::new(shell)
            .arg("-c")
            .arg(command)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_thread = thread::spawn(move || {
            let stdout_reader = BufReader::new(stdout);
            for line in stdout_reader.lines() {
                let line = line.unwrap();
                println!("[cmd]: {}", line);
            }
        });

        let stderr_thread = thread::spawn(move || {
            let stderr_reader = BufReader::new(stderr);
            for line in stderr_reader.lines() {
                let line = line.unwrap();
                println!("[cmd]: {}", line);
            }
        });

        stdout_thread.join().unwrap();
        stderr_thread.join().unwrap();

        let status = child.wait()?;
        std::env::set_current_dir(original_dir)?;

        if status.success() {
            info!("Step '{step_id}' was executed successfully");
            Ok(())
        } else {
            let err = format!("Step '{step_id}' failed with status: {}", status);
            error!("{}", err);
            Err(err.into())
        }
    }
}
