//! Command

use std::env;

use anyhow::Result;
use clap::{Parser, Subcommand};
use docz_lib::{Config, Format, Service};

use crate::args::CliArgs;

/// CLI command
#[derive(Subcommand)]
pub enum Command {
    /// Initializes the project
    Init {},
    /// Builds the project
    Build {
        /// Output format
        #[arg(short, long)]
        out: String,
    },
}

/// Runs the CLI
pub fn run() -> Result<()> {
    let args = CliArgs::parse();

    if args.dbg {
        env_logger::init();
    }

    if let Some(cwd) = &args.cwd {
        env::set_current_dir(cwd)?;
    }

    eprintln!();
    match args.command {
        Command::Init {} => {
            Config::init_dir()?;
            eprintln!("✅ initialized repo");
        }
        Command::Build { out } => {
            let cfg = match &args.cfg {
                Some(cfg_file) => Config::load_from(cfg_file)?,
                _ => Config::load()?,
            };
            let format = out.parse::<Format>()?;
            let service = Service::new(cfg).defaults();
            service.build(format)?;
            eprintln!("✅ built docs");
        }
    }

    Ok(())
}
