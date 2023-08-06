//! Build command

use std::{path::PathBuf, fs};

use anyhow::Result;
use log::debug;
use markdown::{ParseOptions, Constructs};

use crate::cfg::Config;

/// BUilds the project
pub fn build(cfg_file: Option<PathBuf>) -> Result<()> {
    let cfg = if let Some(cfg_file) = &cfg_file {
        Config::load_from(cfg_file)?
    } else {
        Config::load()?
    };
    debug!("CFG {cfg:#?}");

    // get all the files
    let files = get_files(&cfg)?;
    for file in files {
        process_file(&file)?;
    }

    eprintln!("✅ Build OK");
    Ok(())
}

/// Gets all the files 
pub fn get_files(cfg: &Config) -> Result<Vec<PathBuf>> {
    let mut files = vec![];
    for file in &cfg.doc.files {
        let file = cfg.src_dir()?.join(file);
        if !file.exists() {
            return Err(anyhow::anyhow!("file {:?} does not exist", file));
        }
        files.push(file);
    }
    Ok(files)
}

/// Process a file
pub fn process_file(file: &PathBuf) -> Result<markdown::mdast::Node> {
    debug!("Processing {:?}", file);
    let data = fs::read_to_string(file)?;
    let mut opts = ParseOptions::default();
    opts.constructs.frontmatter = true;
    let ast = markdown::to_mdast(&data, &opts).map_err(|e| anyhow::anyhow!("{}", e))?;
    debug!("AST {:#?}", ast);
    Ok(ast)
}