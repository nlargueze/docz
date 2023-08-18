//! Command

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use docz_lib::{build::BuildOptions, serve::ServeOptions, Service};

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
    Build {
        #[arg(long, short)]
        watch: bool,
    },
    /// Cleans the build folder
    Clean {},
    /// Servces the doc
    Serve {
        #[arg(long, short, default_value_t = 3000)]
        port: u16,
        #[arg(long)]
        no_watch: bool,
        #[arg(long)]
        no_open: bool,
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

    eprintln!();
    match args.command {
        Command::Init {} => {
            Service::init_root_dir(&root_dir)?;
            eprintln!("✅ Initialized repo");
        }
        Command::Clean {} => {
            let service = init_service(&root_dir)?;
            service.remove_build_dir()?;
            eprintln!("✅ Cleaned the build folder");
        }
        Command::Build { watch } => {
            let service = init_service(&root_dir)?;
            eprintln!("Building with watch ...");
            service
                .build(BuildOptions {
                    watch,
                    extra_watch_dirs: vec![],
                    on_rebuilt: Some(Box::new(|event| {
                        eprintln!("... Rebuilt ({event:?})",);
                    })),
                })
                .await?;
            eprintln!("✅ Built the docs");
        }
        Command::Serve {
            port,
            no_watch,
            no_open,
        } => {
            let service = init_service(&root_dir)?;
            service
                .serve(ServeOptions {
                    port,
                    watch: !no_watch,
                    open: !no_open,
                    extra_watch_dirs: vec![],
                })
                .await?;
        }
    }

    Ok(())
}

/// Initializes the service
fn init_service(root_dir: &Path) -> Result<Service> {
    let service = Service::builder()
        .root_dir(root_dir)
        .dbg_renderer()
        .html_renderer()
        .build()?;
    Ok(service)
}
