//! Configuration

use std::{fs, env, path::{PathBuf, Path}};

use log::debug;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Error};

/// Configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub doc: DocConfig,
    pub build: BuildConfig,
}

/// General doc configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocConfig {
    pub title: String,
    pub description: String,
    pub authors: Vec<String>,
    pub files: Vec<String>,
}

impl Default for DocConfig {
    fn default() -> Self {
        Self { 
            title: "Title".to_string(), 
            description: String::default(), 
            authors: vec![],
            files: vec![
                "00-intro.md".to_string()
            ]
        }
    }
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub dir: PathBuf,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("build"),
        }
    }
}

impl Config {
    /// Default config file name
    pub const DEFAULT_NAME: &str = "doc.toml";

    /// Loads the configuration
    /// 
    /// The configuration is loaded from the file system. 
    /// The default file name is `doc.toml`, and is located
    /// in the current working directory.
    /// 
    pub fn load() -> Result<Self> {
        let cwd = env::current_dir()?;
        let cfg_file = cwd.join(Self::DEFAULT_NAME);
        debug!("config file: {}", cfg_file.to_string_lossy()); 
        let data = fs::read(&cfg_file)?;
        let data_str = String::from_utf8(data)?;
        toml::from_str::<Config>(&data_str).map_err(|e| e.into())
    }

    /// Loads from a specific file
    pub fn load_from(cfg_file: &Path) -> Result<Self> {
        debug!("config file: {}", cfg_file.to_string_lossy());
        let data = fs::read(&cfg_file)?;
        let data_str = String::from_utf8(data)?;
        toml::from_str::<Config>(&data_str).map_err(|e| e.into())
    } 

    /// Saves the config to a file
    pub fn save(&self) -> Result<()> {
        let data = toml::to_string_pretty(self)?;
        let cwd = env::current_dir()?;
        let cfg_file = cwd.join(Self::DEFAULT_NAME);
        if cfg_file.exists() {
            return Err(Error::msg("config file already exists"));
        }
        fs::write(cfg_file, data)?;
        Ok(())
    }

    /// Returns the root directory
    pub fn root_dir(&self) -> Result<PathBuf> {
        Ok(env::current_dir()?)
    } 

    /// Returns the src directory
    pub fn src_dir(&self) -> Result<PathBuf> {
        let cwd = env::current_dir()?;
        Ok(cwd.join("src"))
    } 

    /// Returns the build directory
    pub fn build_dir(&self) -> Result<PathBuf> {
        let cwd = env::current_dir()?;
        Ok(cwd.join(&self.build.dir))
    } 
}