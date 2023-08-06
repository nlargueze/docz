//! A documenta generator

use std::env;

use clap::Parser;
use anyhow::Result;
use cmd::{CliArgs, Command};

pub mod cmd;
pub mod cfg;

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
            cmd::init::init()?;
        }
        Command::Build {} => {
            cmd::build::build(args.cfg)?;
        }
    }

    Ok(())
}
