//! Command

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use docz_lib::{
    serve::ServeOptions,
    watch::{WatchEvent, WatchOptions},
    Service,
};

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
        /// Watches for changes
        #[arg(long, short)]
        watch: bool,
    },
    /// Cleans the build folder
    Clean {},
    /// Servces the doc
    Serve {
        /// Server port
        #[arg(long, short, default_value_t = 3000)]
        port: u16,
        /// Do not watch
        #[arg(short, long)]
        watch: bool,
        /// Do not open the browser  
        #[arg(short, long)]
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
            let mut service = init_service(&root_dir)?;
            if !watch {
                eprintln!("Building ..");
                service.build()?;
                eprintln!("✅ Built the docs");
            } else {
                eprintln!("Building with watch ...");
                service
                    .build_and_watch(docz_lib::watch::WatchOptions {
                        on_rebuilt: Some(on_rebuilt),
                    })
                    .await?;
            }
        }
        Command::Serve { port, watch, open } => {
            let service = init_service(&root_dir)?;
            if !watch {
                service.serve(ServeOptions { port, open }, None).await?;
            } else {
                service
                    .serve(
                        ServeOptions { port, open },
                        Some(WatchOptions {
                            on_rebuilt: Some(on_rebuilt),
                        }),
                    )
                    .await?;
            }
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
        .epub_renderer()
        .build()?;
    Ok(service)
}

/// Prints a message on rebuild
fn on_rebuilt(event: WatchEvent) {
    eprintln!(
        "... Rebuilt ({})",
        event
            .paths
            .into_iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}
