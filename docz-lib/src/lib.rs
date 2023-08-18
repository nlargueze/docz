//! Docz library

pub mod build;
pub mod cfg;
pub mod rend;
pub mod serve;
pub mod src;
pub mod watch;

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result};

use cfg::Config;
use log::trace;
use rend::{DebugRenderer, HTMLRenderer, Renderer};

/// Documentation service
#[derive(Default)]
pub struct Service {
    /// Config
    config: Config,
    /// Renderers
    renderers: HashMap<String, Box<dyn Renderer + Send + Sync>>,
}

impl Service {
    /// Creates a service
    pub fn builder() -> ServiceBuilder {
        ServiceBuilder::default()
    }

    /// Checks if the root_dir is initialized
    pub fn is_root_dir_init(root_dir: impl AsRef<Path>) -> bool {
        let mut config = Config::default();
        config.set_root_dir(root_dir.as_ref());
        config.file_exists()
    }

    /// Initializes the directory
    pub fn init_root_dir(root_dir: impl AsRef<Path>) -> Result<()> {
        let mut config = Config::default();
        config.set_root_dir(root_dir.as_ref());

        // config file
        let config_file = config.file_path();
        if config_file.exists() {
            return Err(Error::msg("config file already exists"));
        }
        config.save_to_file()?;

        // .gitignore
        let gitignore_file = config.root_dir().join(PathBuf::from(".gitignore"));
        if gitignore_file.exists() {
            return Err(anyhow::anyhow!(".gitignore already exists"));
        }
        fs::write(gitignore_file, "build")?;

        // src dir
        let src_dir = config.src_dir();
        if src_dir.exists() {
            return Err(anyhow::anyhow!("src directory already exists"));
        }
        fs::create_dir(&src_dir)?;

        // dummy file
        let dummy_file = src_dir.join(PathBuf::from("01-chapter_1.md"));
        fs::write(dummy_file, "# Chapter 1\n")?;

        // assets dir
        let assets_dir = config.assets_dir();
        if assets_dir.exists() {
            return Err(anyhow::anyhow!("assets directory already exists"));
        }
        fs::create_dir(assets_dir)?;

        Ok(())
    }
}

/// Service Builder
pub struct ServiceBuilder {
    /// Root dir
    root_dir: PathBuf,
    /// Renderers
    renderers: HashMap<String, Box<dyn Renderer + Send + Sync>>,
}

impl Default for ServiceBuilder {
    fn default() -> Self {
        Self {
            root_dir: env::current_dir().unwrap(),
            renderers: HashMap::new(),
        }
    }
}

impl ServiceBuilder {
    /// Sets the root dir
    pub fn root_dir(mut self, root_dir: impl AsRef<Path>) -> Self {
        self.root_dir = root_dir.as_ref().to_owned();
        self
    }

    /// Adds a renderer
    pub fn renderer(mut self, id: &str, renderer: impl Renderer + Send + Sync + 'static) -> Self {
        self.renderers.insert(id.into(), Box::new(renderer));
        self
    }

    /// Adds the debug renderer
    pub fn dbg_renderer(self) -> Self {
        let dbg_renderer = DebugRenderer::new();
        self.renderer("debug", dbg_renderer)
    }

    /// Adds the HTML renderer
    pub fn html_renderer(self) -> Self {
        let html_renderer = HTMLRenderer::new();
        self.renderer("html", html_renderer)
    }

    /// Builds the service
    pub fn build(mut self) -> Result<Service> {
        let mut service = Service::default();
        service.config.set_root_dir(&self.root_dir);
        service.config.load_file()?;
        trace!("Service root is: {}", service.config.root_dir().display());
        for (id, renderer) in self.renderers.iter_mut() {
            trace!("Registering renderer ({id})");
            renderer.register(&service.config)?;
        }
        service.renderers = self.renderers;
        Ok(service)
    }
}
