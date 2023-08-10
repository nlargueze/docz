//! Command

use std::env;

use anyhow::Result;
use clap::Parser;
use docz_lib::{Config, Format, Service};

use crate::args::{CliArgs, Command};

/// Runs the service
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
        Command::Build { out: _ } => {
            let cfg = match &args.cfg {
                Some(cfg_file) => Config::load_from(cfg_file)?,
                _ => Config::load()?,
            };

            let service = Service::new(cfg).defaults();
            service.build(Format::Html)?;
            eprintln!("✅ built docs");
        }
    }

    Ok(())
}
