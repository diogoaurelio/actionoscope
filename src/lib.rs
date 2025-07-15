use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Formatter, Pointer};
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::{collections, thread};

mod github;
pub mod models;

use crate::models::github_workflows::{Step, Trigger};
use github::metadata::get_git_repo_vars;

pub trait CommandRunner {
    fn run(&self, step: &Step) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubStepCommandRunner {
    pub env_vars: Option<collections::HashMap<String, String>>,
    pub secret_vars: Option<collections::HashMap<String, String>>,
}

impl GithubStepCommandRunner {
    pub fn new(
        env_vars: Option<collections::HashMap<String, String>>,
        secret_vars: Option<collections::HashMap<String, String>>,
    ) -> Self {
        Self {
            env_vars,
            secret_vars,
        }
    }

    pub fn replace_env_vars(&self, command: &str) -> String {
        let mut result = command.to_string();

        if let Some(env_vars) = self.env_vars.to_owned() {
            let re = regex::Regex::new(r"\$\{\{\s*env\.(\w+)\s*\}\}").unwrap();
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    env_vars
                        .get(&caps[1])
                        .cloned()
                        .or_else(|| std::env::var(&caps[1]).ok())
                        .unwrap_or_else(|| "".to_string())
                })
                .to_string();
        }

        if let Some(secret_vars) = self.secret_vars.to_owned() {
            let re = regex::Regex::new(r"\$\{\{\s*secrets\.(\w+)\s*\}\}").unwrap();
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    secret_vars
                        .get(&caps[1])
                        .cloned()
                        .unwrap_or_else(|| "".to_string())
                })
                .to_string();
        }

        let git_vars = match get_git_repo_vars() {
            Ok(vars) => Some(vars),
            Err(e) => {
                debug!("failed to extract git metadata: {}", e);
                None
            }
        };

        if let Some(git_vars) = git_vars.to_owned() {
            let re = regex::Regex::new(r"\$\{\{\s*github\.(\w+)\s*\}\}").unwrap();
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    git_vars
                        .get(&caps[1])
                        .cloned()
                        .unwrap_or_else(|| "".to_string())
                })
                .to_string();
        }

        result
    }
}

impl CommandRunner for GithubStepCommandRunner {
    fn run(&self, step: &Step) -> Result<(), Box<dyn std::error::Error>> {
        let step_id = step.get_name_or_id();
        if step.run.is_none() {
            if step.uses.is_none() {
                let err = format!("No run command found for step id/name '{step_id}'");
                error!(
                    "{}; Step details are:\nname: {}\nid: {}\nuses: {}\nshell: {}",
                    err,
                    step.name.as_deref().unwrap_or("NA"),
                    step.id.as_deref().unwrap_or("NA"),
                    step.uses.as_deref().unwrap_or("NA"),
                    step.shell.as_deref().unwrap_or("NA")
                );
                return Err(err.into());
            } else {
                warn!(
                    "Currently, 'uses' is not supported. Skipping step '{}'",
                    step_id
                );
                return Ok(());
            }
        }

        let git_vars = match get_git_repo_vars() {
            Ok(vars) => Some(vars),
            Err(e) => {
                debug!("failed to extract git metadata: {}", e);
                None
            }
        };

        let command = step.run.as_deref().unwrap();
        let command = self.replace_env_vars(command).trim().to_string();

        let shell = step.shell.as_deref().unwrap_or("bash");
        let original_dir = std::env::current_dir()?;

        if step.working_directory.is_some() {
            info!(
                "Changing working directory to: {}/{}",
                original_dir.display(),
                step.working_directory.as_deref().unwrap()
            );
            std::env::set_current_dir(step.working_directory.as_deref().unwrap())?;
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
                println!("[stdout]: {line}");
            }
        });

        let stderr_thread = thread::spawn(move || {
            let stderr_reader = BufReader::new(stderr);
            for line in stderr_reader.lines() {
                let line = line.unwrap();
                println!("[stdout]: {line}");
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
            let err = format!("Step '{step_id}' failed with status: {status}");
            error!("{}", err);
            Err(err.into())
        }
    }
}
