//! Init command

use std::fs;

use anyhow::Result;

use crate::cfg::Config;

/// Initializes the project
pub fn init() -> Result<()> {
    // create and save the config file
    let cfg = Config::default();
    cfg.save()?;

    // create the `src` folder
    let src_dir = cfg.src_dir()?;
    if src_dir.exists() {
        return Err(anyhow::anyhow!("src directory already exists"));
    }
    fs::create_dir(cfg.src_dir()?)?;

    // create the .gitignore
    fs::write(".gitignore", "build")?;

    eprintln!("✅ initialized project");
    Ok(())
}