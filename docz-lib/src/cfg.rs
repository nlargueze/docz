//! Configuration

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Service configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Root directory
    root_dir: PathBuf,
    /// Config file
    file: ConfigFile,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_dir: env::current_dir().unwrap(),
            file: ConfigFile::default(),
        }
    }
}

impl Config {
    /// Config file name
    pub(crate) const FILE_NAME: &str = "doc.toml";

    /// Creates a new default config
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the root dir
    pub fn set_root_dir(&mut self, root_dir: &Path) -> &mut Self {
        self.root_dir = root_dir.to_owned();
        self
    }

    /// Returns the root dir
    pub fn root_dir(&self) -> PathBuf {
        self.root_dir.to_owned()
    }

    /// Returns the config file
    pub fn file(&self) -> &ConfigFile {
        &self.file
    }

    /// Returns the path to the config file
    pub fn file_path(&self) -> PathBuf {
        self.root_dir.join(Self::FILE_NAME)
    }

    /// Returns the source directory
    pub fn src_dir(&self) -> PathBuf {
        self.root_dir.join(&self.file.src.src_dir)
    }

    /// Returns the assets directory
    pub fn assets_dir(&self) -> PathBuf {
        self.src_dir().join(&self.file.src.assets_dir)
    }

    /// Returns the build directory
    pub fn build_dir(&self) -> PathBuf {
        self.root_dir.join(&self.file.build.build_dir)
    }

    /// Loads the configuration from a file
    pub(crate) fn load_file(&mut self) -> Result<()> {
        let path = self.file_path();
        let data = fs::read(path).context("config file not found")?;
        let data_str = String::from_utf8(data)?;
        let file = toml::from_str::<ConfigFile>(&data_str)?;
        self.file = file;
        Ok(())
    }

    /// Saves the config to a file
    pub(crate) fn save_to_file(&self) -> Result<()> {
        let path = self.file_path();
        let data = toml::to_string_pretty(&self.file)?;
        fs::write(path, data)?;
        Ok(())
    }

    /// Returns the output IDS
    pub fn output_ids(&self) -> Vec<&str> {
        self.file.output.keys().map(|x| x.as_str()).collect()
    }

    /// Returns the config for a specific output
    pub fn get_output_cfg<T>(&self, id: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let value = match self.file.output.get(id) {
            Some(x) => x,
            None => return Ok(None),
        };

        let t = toml::from_str::<T>(&value.to_string())?;
        Ok(Some(t))
    }
}

/// Configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    /// Documentation configuration
    pub doc: DocConfig,
    /// Source config
    pub src: SourceConfig,
    /// Build config
    pub build: BuildConfig,
    /// Output config
    pub output: HashMap<String, toml::Value>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        let mut output = HashMap::new();
        output.insert("html".to_string(), toml::Value::Table(toml::Table::new()));
        output.insert("debug".to_string(), toml::Value::Table(toml::Table::new()));

        Self {
            doc: DocConfig::default(),
            src: SourceConfig::default(),
            build: BuildConfig::default(),
            output,
        }
    }
}

/// Documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocConfig {
    /// Doc title
    pub title: String,
    /// Doc description
    pub description: String,
    /// Authors
    pub authors: Vec<String>,
}

impl Default for DocConfig {
    fn default() -> Self {
        Self {
            title: "Doc title".to_string(),
            description: "Doc description".to_string(),
            authors: vec![],
        }
    }
}

/// Source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    /// Source files directory
    pub src_dir: PathBuf,
    /// Assets files directory
    pub assets_dir: PathBuf,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            src_dir: PathBuf::from("src"),
            assets_dir: PathBuf::from("_assets"),
        }
    }
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub build_dir: PathBuf,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            build_dir: PathBuf::from("build"),
        }
    }
}
