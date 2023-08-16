//! Docz library

pub mod build;
pub mod cfg;
pub mod doc;
pub mod rend;
pub mod serve;
pub mod watch;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result};

use cfg::Config;
use rend::Renderer;

/// Documentation service
#[derive(Default)]
pub struct Service {
    /// Config
    config: Config,
    /// Renderers
    renderers: HashMap<String, Box<dyn Renderer>>,
}

impl Service {
    /// Creates a service builder
    pub fn builder() -> ServiceBuilder {
        ServiceBuilder::default()
    }

    /// Initializes the directory
    pub fn init_root_dir(&self) -> Result<()> {
        // config file
        let config_file = self.config.file_path();
        if config_file.exists() {
            return Err(Error::msg("config file already exists"));
        }
        self.config.save_to_file()?;

        // src dir
        let src_dir = self.config.src_dir();
        if src_dir.exists() {
            return Err(anyhow::anyhow!("src directory already exists"));
        }
        fs::create_dir(src_dir)?;

        // .gitignore
        let gitignore_file = self.config.root_dir().join(PathBuf::from(".gitignore"));
        if gitignore_file.exists() {
            return Err(anyhow::anyhow!(".gitignore already exists"));
        }
        fs::write(gitignore_file, "build")?;

        // assets dir
        let assets_dir = self.config.assets_dir();
        if assets_dir.exists() {
            return Err(anyhow::anyhow!("assets directory already exists"));
        }
        fs::create_dir(assets_dir)?;

        Ok(())
    }
}

/// Service Builder
#[derive(Default)]
pub struct ServiceBuilder {
    /// Root dir
    root_dir: PathBuf,
    /// Renderers
    renderers: HashMap<String, Box<dyn Renderer>>,
}

impl ServiceBuilder {
    /// Sets the root dit
    pub fn root_dir(mut self, root_dir: impl AsRef<Path>) -> Self {
        self.root_dir = root_dir.as_ref().to_owned();
        self
    }

    /// Adds a renderer
    pub fn renderer(mut self, renderer: impl Renderer + Send + Sync + 'static) -> Self {
        let id = renderer.id().to_owned();
        self.renderers.insert(id, Box::new(renderer));
        self
    }

    /// Builds the service
    pub fn build(self) -> Result<Service> {
        let mut service = Service::default();
        service.config.set_root_dir(&self.root_dir);
        service.config.load_file()?;
        service.renderers = self.renderers;
        Ok(service)
    }
}
