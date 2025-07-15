use clap::{Parser, Subcommand};
use env_logger::{Builder, Target};
use std::env;

use actionoscope::{ls_command, run_command};

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
