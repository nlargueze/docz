//! Command

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use docz_lib::{serve::ServeOptions, Service};

/// CLI arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Optional current working directory
    #[arg(long)]
    pub cwd: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub dbg: bool,

    /// Commands
    #[command(subcommand)]
    pub command: Command,
}

/// CLI command
#[derive(Subcommand)]
pub enum Command {
    /// Initializes the root directory
    Init {},
    /// Builds the doc
    Build {},
    /// Servces the doc
    Serve {
        #[arg(long, short, default_value_t = 3000)]
        port: u16,
        #[arg(long, short, default_value_t = true)]
        watch: bool,
        #[arg(long, short, default_value_t = true)]
        open: bool,
    },
}

/// Runs the CLI
pub async fn run() -> Result<()> {
    let args = CliArgs::parse();

    if args.dbg {
        env_logger::init();
    }

    let root_dir = match &args.cwd {
        Some(cwd) => cwd.to_owned(),
        None => env::current_dir().unwrap(),
    };

    // init service

    eprintln!();
    match args.command {
        Command::Init {} => {
            Service::init_root_dir(&root_dir)?;
            eprintln!("✅ initialized repo");
        }
        Command::Build {} => {
            let service = init_service(&root_dir)?;
            service.build()?;
            eprintln!("✅ built docs");
        }
        Command::Serve { port, watch, open } => {
            let service = init_service(&root_dir)?;
            service.serve(ServeOptions { port, watch, open }).await?;
        }
    }

    Ok(())
}

/// Initializes the service
fn init_service(root_dir: &Path) -> Result<Service> {
    let service = Service::builder()
        .root_dir(root_dir)
        .dbg_renderer()
        .html_renderer()?
        .build()?;
    Ok(service)
}
