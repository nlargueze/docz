//! Build

use std::{fs, path::Path};

use anyhow::{anyhow, Result};
use log::trace;

use crate::{
    doc::{Document, Section},
    Service,
};

impl Service {
    /// Removes the build folder
    pub fn remove_build_dir(&self) -> Result<()> {
        let build_dir = self.config.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(build_dir)?
        }
        Ok(())
    }

    /// Builds the documentation
    pub fn build(&self) -> Result<()> {
        let doc = self.extract_src()?;

        // recreate the build dir
        let build_dir = self.config.build_dir();
        self.remove_build_dir()?;
        fs::create_dir(&build_dir)?;

        for id in self.config.output_ids() {
            if let Some(renderer) = self.renderers.get(id) {
                renderer.render(&self.config, &doc)?;
            } else {
                return Err(anyhow!(
                    "Invalid output type ({}). Check the config file or add a renderer",
                    id
                ));
            };
        }

        Ok(())
    }

    /// Extracts the source files and populates the [Document]
    fn extract_src(&self) -> Result<Document> {
        let mut doc = Document::default();
        let src_dir = self.config.src_dir();
        let sections = self.extract_src_section_iter(&src_dir, true)?;
        doc.sections = sections;
        Ok(doc)
    }

    /// Extracts the sections for a dir recursively
    fn extract_src_section_iter(&self, dir: &Path, is_top_level: bool) -> Result<Vec<Section>> {
        let assets_dir = self.config.assets_dir();

        let mut files = vec![];
        let mut dirs = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let entry_type = entry.file_type()?;

            if is_top_level && entry.path().starts_with(&assets_dir) {
                continue;
            }

            if entry_type.is_dir() {
                dirs.push(entry.path());
            }

            if entry_type.is_file() {
                files.push(entry.path());
            }
        }

        let mut sections = vec![];
        for file in &files {
            let mut section = Section::new(file);
            section.content = fs::read(file.as_path())?;

            // populate subsections
            if let Some(index) = dirs.iter().position(|dir| {
                // NB: the dir is related to the file if it has the same name (without extension)
                let mut file_no_ext = file.clone();
                file_no_ext.set_extension("");
                dir.file_name() == file_no_ext.file_name()
            }) {
                let dir = dirs.remove(index);
                let sub_sections = self.extract_src_section_iter(&dir, false)?;
                section.children = sub_sections;
            }

            sections.push(section);
        }

        // check that no sub-dirs is orphan
        for dir in dirs {
            trace!("orphan source dir: {:#?}", dir);
        }

        Ok(sections)
    }
}
