use clap::Parser;
use env_logger::{Builder, Target};
use log::{error, info};
use std::path::{Path, PathBuf};
use std::{env, fs};

use actionoscope::Workflow;

#[derive(Debug, Parser)]
#[command(name = "actionoscope")]
#[command(
    about = "Run steps from a GitHub Actions workflow locally.",
    version = "1.0"
)]
struct Cli {
    /// Path to the workflow YAML file
    #[arg(long, short = 'w')]
    workflow_file: Option<String>,

    /// Job name to run
    #[arg(long, short = 'j')]
    job: String,

    /// Step name or id to run
    #[arg(long, short = 's')]
    step: Option<String>,

    /// Step name or id to start running from
    #[arg(long, short = 'f')]
    from_step: Option<String>,

    /// Step name or id to start running from
    #[arg(long, short = 't')]
    to_step: Option<String>,
}

fn validate_workflow_file(workflows_dir: &Path, workflow_file: &str) -> Option<PathBuf> {
    let workflow_path = workflows_dir.join(workflow_file);
    if workflow_path.exists() {
        Some(workflow_path)
    } else {
        None
    }
}

fn find_workflow_files(
    workflow_file: Option<String>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let current_dir = Path::new(".");
    let workflows_dir = current_dir.join(".github").join("workflows");
    let mut result = Vec::new();
    if let Some(workflow_file) = workflow_file {
        // check if the file exists in the current directory
        let workflow_path = validate_workflow_file(current_dir, &workflow_file);
        if let Some(path) = workflow_path {
            result.push(path);
            return Ok(result);
        }

        // fallback: check if the file exists in the .github/workflows directory
        let exists = validate_workflow_file(&workflows_dir, &workflow_file);
        if let Some(path) = exists {
            result.push(path);
            return Ok(result);
        }
        let err = format!("Provided workflow file {} was not found", workflow_file);
        return Err(err.into());
    }
    for entry in fs::read_dir(workflows_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            result.push(path);
        }
    }

    if !result.is_empty() {
        Ok(result)
    } else {
        Err("No workflow files found in .github/workflows directory".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("RUST_LOG").is_err() {
        Builder::new()
            .target(Target::Stdout)
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        env_logger::init();
    }

    let cli = Cli::parse();
    if cli.step.is_none() && cli.from_step.is_none() {
        error!("Error: Either 'step' or 'from-step' argument must be specified.");
        std::process::exit(1);
    }

    let workflow_files = find_workflow_files(cli.workflow_file)?;

    info!(
        "Found workflow file(s): {}",
        workflow_files
            .iter()
            .map(|w| w.to_string_lossy())
            .collect::<Vec<_>>()
            .join(", ")
    );

    for workflow_file in &workflow_files {
        let workflow_content = fs::read_to_string(workflow_file.to_string_lossy().into_owned())
            .unwrap_or_else(|err| {
                error!("Failed to read the workflow file: {}", err);
                std::process::exit(1);
            });

        let workflow = Workflow::from_yaml(&workflow_content).unwrap_or_else(|err| {
            error!("Failed to parse the workflow file: {}", err);
            std::process::exit(1);
        });

        let job = workflow.get_job(&cli.job).unwrap_or_else(|| {
            error!("Job '{}' not found in the workflow", cli.job);
            std::process::exit(1);
        });

        let step_name: &str = &cli.step.as_deref().or(cli.from_step.as_deref()).unwrap();
        let first_step = job.get_step(step_name).unwrap_or_else(|| {
            error!("Step '{}' not found in the job '{}'", step_name, cli.job);
            std::process::exit(1);
        });

        if cli.step.is_some() {
            first_step.run_cmd(workflow.env.clone())?;
        } else {
            for step in &job.get_all_steps_since(step_name, cli.to_step.as_deref()) {
                if let Err(e) = step.run_cmd(workflow.env.clone()) {
                    error!("Error running step '{}': {}", step.get_name_or_id(), e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
