//! Arguments

use std::path::PathBuf;

use clap::Parser;

use crate::cmd::Command;

/// CLI arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Optional current working directory
    #[arg(long)]
    pub cwd: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub cfg: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub dbg: bool,

    /// Commands
    #[command(subcommand)]
    pub command: Command,
}
