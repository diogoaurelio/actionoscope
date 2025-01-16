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
    workflow_file: String,

    /// Job name to run
    #[arg(long, short = 'j')]
    job: String,

    /// Step name or id to run
    #[arg(long, short = 's')]
    step: Option<String>,

    /// Step name or id to start running from
    #[arg(long, short = 'f')]
    from_step: Option<String>,
}

fn validate_workflow_file(workflows_dir: &Path, workflow_file: &str) -> Option<PathBuf> {
    let workflow_path = workflows_dir.join(workflow_file);
    if workflow_path.exists() {
        Some(workflow_path)
    } else {
        None
    }
}

fn find_workflow_file(workflow_file: &str) -> Option<PathBuf> {
    let current_dir = Path::new(".");
    let workflow_path = validate_workflow_file(&current_dir, workflow_file);

    if workflow_path.is_some() {
        return workflow_path;
    }

    let workflows_dir = current_dir.join(".github").join("workflows");

    validate_workflow_file(&workflows_dir, workflow_file)
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

    let workflow_file = find_workflow_file(&cli.workflow_file).unwrap_or_else(|| {
        error!("Workflow file '{}' not found", &cli.workflow_file);
        std::process::exit(1);
    });

    info!("Found workflow file: {}", workflow_file.to_string_lossy());

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
        for step in &job.get_all_steps_since(step_name) {
            if let Err(e) = step.run_cmd(workflow.env.clone()) {
                error!("Error running step '{}': {}", step.get_name_or_id(), e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
