use clap::{Parser, Subcommand};
use env_logger::{Builder, Target};
use log::{error, info};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use actionoscope::models::github_workflows::{Job, Workflow};
use actionoscope::{CommandRunner, GithubStepCommandRunner};

#[derive(Debug, Parser)]
#[command(name = "actionoscope")]
#[command(
    about = "Run steps from a GitHub Actions workflow locally.",
    version = "1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run a job or step
    Run {
        /// Path to the workflow YAML file
        #[arg(long, short = 'w')]
        workflow_file: Option<String>,

        /// Job name to run
        #[arg(long, short = 'j')]
        job: Option<String>,

        /// Step name or id to run
        #[arg(
            long,
            short = 's',
            help = "Provide a step name or id to specify which step should actionoscope run"
        )]
        step: Option<String>,

        /// Step name or id to start running from
        #[arg(
            long,
            short = 'f',
            help = "Provide a step name or id to specify from which step should actionoscope start running"
        )]
        from_step: Option<String>,

        /// Step name or id to start running from
        #[arg(
            long,
            short = 't',
            help = "Provide a step name or id to specify until which step should actionoscope run"
        )]
        to_step: Option<String>,

        #[arg(
            long,
            short = 'e',
            help = "Path to the .env file that serves as the secrets file"
        )]
        secrets_file: Option<String>,

        /// Skip one or more steps
        #[arg(long, short = 'k', help = "Steps to skip")]
        skip_step: Vec<String>,
    },
    /// List workflow files
    Ls {
        /// Path to the workflow YAML file
        #[arg(long, short = 'w')]
        workflow_file: Option<String>,
    },
}

struct RunJobConfig<'a> {
    jobs: Vec<&'a Job>,
    job_names: Vec<String>,
    step: Option<String>,
    from_step: Option<String>,
    to_step: Option<String>,
    env_vars: Option<HashMap<String, String>>,
    secret_vars: Option<HashMap<String, String>>,
    steps_to_skip: Vec<String>,
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
        let err = format!("Provided workflow file {workflow_file} was not found");
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

fn run_jobs(config: RunJobConfig) -> Result<(), Box<dyn std::error::Error>> {
    let jobs = &config.jobs;
    let command_runner =
        GithubStepCommandRunner::new(config.env_vars.clone(), config.secret_vars.clone());
    for (index, job) in jobs.iter().enumerate() {
        info!("Running job '{}'", &config.job_names[index]);
        if config.step.is_some() {
            let step_name = &config.step.clone().unwrap();
            let step = job.get_step(step_name).unwrap_or_else(|| {
                error!("Step '{}' not found in the job '{:?}'", step_name, job);
                std::process::exit(1);
            });
            command_runner.run(step)?;
        } else {
            if config.from_step.is_some()
                && job.get_step(&config.from_step.clone().unwrap()).is_none()
            {
                error!(
                    "from-step '{}' not found in the job '{}'",
                    &config.from_step.clone().unwrap(),
                    &config.job_names[index]
                );
                std::process::exit(1);
            }
            if config.to_step.is_some() && job.get_step(&config.to_step.clone().unwrap()).is_none()
            {
                error!(
                    "to-step '{}' not found in the job '{}'",
                    &config.to_step.clone().unwrap(),
                    &config.job_names[index]
                );
                std::process::exit(1);
            }
            for step in
                &job.get_all_steps_since(config.from_step.as_deref(), config.to_step.as_deref())
            {
                if config
                    .steps_to_skip
                    .iter()
                    .any(|s| s == step.get_id() || s == step.get_name())
                {
                    info!("Skipping step '{}'", step.get_name_or_id());
                    continue;
                }
                if let Err(e) = command_runner.run(step) {
                    error!("Error running step '{}': {}", step.get_name_or_id(), e);
                    std::process::exit(1);
                }
            }
        }
    }
    Ok(())
}

fn ls_command(workflow_file: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let workflow_files = find_workflow_files(workflow_file.clone())?;

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
        println!("{workflow:#?}");
    }
    Ok(())
}

fn load_env_vars(env_file: Option<&str>) -> Option<std::collections::HashMap<String, String>> {
    if let Some(file) = env_file {
        dotenv::from_filename(file).ok()?;
        let env_vars: std::collections::HashMap<String, String> = env::vars().collect();
        Some(env_vars)
    } else {
        None
    }
}

fn run_command(
    workflow_file: Option<String>,
    job: Option<String>,
    step: Option<String>,
    from_step: Option<String>,
    to_step: Option<String>,
    secrets_file: Option<String>,
    steps_to_skip: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let workflow_files = find_workflow_files(workflow_file.clone())?;
    let secrets = load_env_vars(secrets_file.as_deref());

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

        let mut jobs: Vec<&Job> = Vec::new();
        let mut job_names: Vec<String> = Vec::new();
        if job.is_some() {
            let job_name = &job.clone().unwrap();
            let job = workflow.get_job(job_name).unwrap_or_else(|| {
                error!("Job '{}' not found in the workflow", job_name);
                std::process::exit(1);
            });
            job_names.push(job_name.to_string());
            jobs.push(job);
        } else {
            for (name, job) in &workflow.jobs {
                job_names.push(name.clone());
                jobs.push(job);
            }
        }
        let job_config = RunJobConfig {
            jobs: jobs.clone(),
            job_names: job_names.clone(),
            step: step.clone(),
            from_step: from_step.clone(),
            to_step: to_step.clone(),
            env_vars: workflow.env.clone(),
            secret_vars: secrets.clone(),
            steps_to_skip: steps_to_skip.to_owned(),
        };

        run_jobs(job_config)?;
    }

    Ok(())
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

    match &cli.command {
        Commands::Run {
            job,
            workflow_file,
            step,
            from_step,
            to_step,
            secrets_file,
            skip_step: steps_to_skip,
            ..
        } => run_command(
            workflow_file.clone(),
            job.clone(),
            step.clone(),
            from_step.clone(),
            to_step.clone(),
            secrets_file.clone(),
            steps_to_skip.to_owned(),
        ),
        Commands::Ls { workflow_file } => ls_command(workflow_file.clone()),
    }
}
