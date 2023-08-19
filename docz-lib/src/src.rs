//! Source

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use log::trace;

use crate::Service;

/// A representation of the source directory
#[derive(Debug, Clone, Default)]
pub struct SourceData {
    /// Files
    pub files: Vec<SourceFile>,
    /// Static assets
    pub assets: Vec<PathBuf>,
}

impl SourceData {
    /// Adds a file
    pub fn add_file(&mut self, file: SourceFile) -> &mut Self {
        self.files.push(file);
        self
    }
}

/// Source file
#[derive(Clone, Default)]
pub struct SourceFile {
    /// Path
    pub path: PathBuf,
    /// Content
    pub content: Vec<u8>,
    /// Children
    pub children: Vec<SourceFile>,
}

impl std::fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content_str =
            String::from_utf8(self.content.to_vec()).unwrap_or("_binary_".to_string());

        f.debug_struct("Section")
            .field("file", &self.path)
            .field("content", &content_str)
            .field("children", &self.children)
            .finish()
    }
}

impl SourceFile {
    /// Creates a new [SourceFile]
    pub fn new(src_path: impl Into<PathBuf>) -> Self {
        Self {
            path: src_path.into(),
            content: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Adds a child
    pub fn add_child(&mut self, child: SourceFile) -> &mut Self {
        self.children.push(child);
        self
    }
}

impl Service {
    /// Loads the source directory
    pub(crate) fn load_src_dir(&self) -> Result<SourceData> {
        let src_dir = self.config.src_dir();
        let assets_dir = self.config.src_assets_dir();
        let files = self.load_src_dir_iter(&src_dir, &[&assets_dir])?;
        let assets = self.load_assets_dir_iter(&assets_dir)?;
        let src_tree = SourceData { files, assets };
        Ok(src_tree)
    }

    /// Loads a source directory recursively
    #[allow(clippy::only_used_in_recursion)]
    fn load_src_dir_iter(&self, dir: &Path, exc_dirs: &[&Path]) -> Result<Vec<SourceFile>> {
        let mut files = vec![];
        let mut dirs = vec![];
        'loop_entry: for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let entry_type = entry.file_type()?;

            for exc_dir in exc_dirs {
                if entry.path().starts_with(exc_dir) {
                    continue 'loop_entry;
                }
            }

            if entry_type.is_dir() {
                dirs.push(entry.path());
            }

            if entry_type.is_file() {
                files.push(entry.path());
            }
        }

        let mut src_files = vec![];
        for file in &files {
            let path = file.to_owned();
            let content = fs::read(file.as_path())?;
            let mut children = vec![];

            // children
            if let Some(index) = dirs.iter().position(|dir| {
                // NB: the dir is related to the file if it has the same name (without extension)
                let mut file_no_ext = file.clone();
                file_no_ext.set_extension("");
                dir.file_name() == file_no_ext.file_name()
            }) {
                let dir = dirs.remove(index);
                children = self.load_src_dir_iter(&dir, &[])?;
            }

            src_files.push(SourceFile {
                path,
                content,
                children,
            });
        }

        // check that no sub-dirs is orphan
        for dir in dirs {
            trace!("orphan source dir: {:#?}", dir);
        }

        Ok(src_files)
    }

    /// Loads the static assets recursively
    #[allow(clippy::only_used_in_recursion)]
    fn load_assets_dir_iter(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut assets = vec![];
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let mut new_assets = self.load_assets_dir_iter(&path)?;
                    assets.append(&mut new_assets);
                } else {
                    assets.push(path);
                }
            }
        }
        Ok(assets)
    }
}
