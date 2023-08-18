//! HTML renderer

use std::{ffi::OsStr, fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions};
use fs_extra::dir::CopyOptions;
use handlebars::Handlebars;
use log::{debug, trace};
use serde::{Deserialize, Serialize};

use crate::{
    cfg::Config,
    src::{SourceData, SourceFile},
};

use super::Renderer;

/// Default template ID
const DEFAULT_TEMPLATE_ID: &str = "default";

/// Renderer for HTML docs
#[derive(Debug)]
pub struct HTMLRenderer {
    registry: Handlebars<'static>,
    template_id: &'static str,
    static_files: Vec<(&'static str, &'static [u8])>,
    static_files_copy: Vec<(PathBuf, PathBuf)>,
}

/// HTML Template
#[derive(Debug)]
struct HTMLTemplate {
    /// ID
    id: &'static str,
    /// Template file
    file: String,
    /// Embedded Static files
    static_files: Vec<(&'static str, &'static [u8])>,
}

/// HTML renderer Config
#[derive(Debug, Deserialize, Default)]
pub struct OutputHtmlConfig {
    /// Overwrites the default template
    pub template: Option<String>,
    /// Overwrites the `index.hbs` file
    pub index: Option<PathBuf>,
    /// Overwrites the static files
    pub static_files: Option<Vec<(PathBuf, PathBuf)>>,
}

/// HTML data
#[derive(Debug, Default, Serialize)]
struct HTMLData {
    pub title: String,
    pub sections: Vec<HTMLDataSection>,
}

/// HTML data section
#[derive(Debug, Default, Serialize)]
struct HTMLDataSection {
    html: String,
}

impl HTMLRenderer {
    /// Creates a new HTML renderer
    pub fn new() -> Self {
        Self {
            registry: Handlebars::new(),
            template_id: DEFAULT_TEMPLATE_ID,
            static_files: vec![],
            static_files_copy: vec![],
        }
    }
}

impl Default for HTMLRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for HTMLRenderer {
    fn register(&mut self, cfg: &Config) -> Result<()> {
        // get config
        let html_config = cfg
            .get_output_cfg::<OutputHtmlConfig>("html")?
            .unwrap_or_default();
        let root_dir = cfg.root_dir();

        // assign template
        let template_id = html_config
            .template
            .unwrap_or(DEFAULT_TEMPLATE_ID.to_string());
        let mut template = match template_id.as_str() {
            "default" => HTMLTemplate {
                id: DEFAULT_TEMPLATE_ID,
                file: include_str!("html/tpl/default/index.hbs").to_string(),
                static_files: vec![
                    ("sse.js", include_bytes!("html/tpl/default/sse.js")),
                    ("style.css", include_bytes!("html/tpl/default/style.css")),
                ],
            },
            _ => {
                return Err(anyhow!("Unknown template ID: {}", template_id));
            }
        };

        // assign template ID
        trace!("Registered HTML template id: {}", template.id);
        self.template_id = template.id;

        // overwrite the index template
        if let Some(index) = html_config.index {
            let index_hbs_path = root_dir.join(index);
            trace!(
                "HTML template, overwriting index file: {}",
                index_hbs_path.display()
            );
            if index_hbs_path.extension() != Some(OsStr::new("hbs")) {
                return Err(anyhow!(
                    "Invalid index template file: {}",
                    index_hbs_path.display()
                ));
            }
            let index_hbs_str = fs::read_to_string(&index_hbs_path).context(format!(
                "HTML template file not found ({})",
                index_hbs_path.display()
            ))?;
            template = HTMLTemplate {
                id: "user-defined",
                file: index_hbs_str,
                static_files: vec![],
            };
        }

        // static files to copy
        if let Some(static_files) = html_config.static_files {
            self.static_files_copy = static_files;
        }

        // assign to renderer
        self.registry
            .register_template_string(template.id, template.file)?;
        self.static_files = template.static_files;

        Ok(())
    }

    fn render(&self, cfg: &Config, data: &SourceData) -> Result<()> {
        // process source
        let html_data = self.process_data(cfg, data)?;
        debug!("HTML template data \n{html_data:#?}");

        // create build dir (NB: /build has been cleared before)
        let root_dir = cfg.root_dir();
        let build_dir = cfg.build_dir().join("html");
        fs::create_dir_all(&build_dir)?;

        // write index.html
        let index_file_str = self.registry.render(self.template_id, &html_data)?;
        let index_file = build_dir.join("index.html");
        fs::write(index_file, index_file_str)?;

        // static files
        for (file_name, file_data) in &self.static_files {
            fs::write(&build_dir.join(file_name), file_data)?;
        }
        for (src, dest) in &self.static_files_copy {
            fs::copy(&root_dir.join(src), &build_dir.join(dest))?;
        }

        // copy assets from source
        let src_assets_dir = cfg.assets_dir();
        fs_extra::copy_items(&[&src_assets_dir], build_dir, &CopyOptions::new())?;
        Ok(())
    }
}

impl HTMLRenderer {
    /// Returns the comrak options
    fn comrak_options(&self) -> ComrakOptions {
        ComrakOptions {
            extension: ComrakExtensionOptions {
                strikethrough: false,
                tagfilter: false,
                table: false,
                autolink: false,
                tasklist: false,
                superscript: false,
                header_ids: None,
                footnotes: false,
                description_lists: false,
                front_matter_delimiter: Some("---".to_string()),
            },
            parse: ComrakParseOptions {
                smart: false,
                default_info_string: None,
                relaxed_tasklist_matching: false,
            },
            render: ComrakRenderOptions {
                hardbreaks: false,
                github_pre_lang: true,
                full_info_string: true,
                width: 0,
                unsafe_: false,
                escape: false,
                list_style: comrak::ListStyleType::Dash,
                sourcepos: false,
            },
        }
    }

    /// Extracts the HTML data
    fn process_data(&self, cfg: &Config, data: &SourceData) -> Result<HTMLData> {
        let mut html_data = HTMLData {
            title: cfg.file().doc.title.to_string(),
            sections: vec![],
        };

        let comrak_opts = self.comrak_options();
        for src_file in &data.files {
            let html_section = self.process_data_section_iter(src_file, &comrak_opts)?;
            html_data.sections.push(html_section);
        }

        Ok(html_data)
    }

    /// Renders a section recursively
    #[allow(clippy::only_used_in_recursion)]
    fn process_data_section_iter(
        &self,
        src_file: &SourceFile,
        comrak_opts: &ComrakOptions,
    ) -> Result<HTMLDataSection> {
        let src_file_html = match src_file
            .path
            .extension()
            .map(|ext| ext.to_str().unwrap_or(""))
            .unwrap_or("")
        {
            "md" | "markdown" => {
                let content_str = String::from_utf8(src_file.content.to_vec())?;
                comrak::markdown_to_html(&content_str, comrak_opts)
            }
            _ => {
                debug!("Skipped file {}", src_file.path.display());
                String::new()
            }
        };

        let mut html_sections = vec![];
        for src_file in &src_file.children {
            let html_section = self.process_data_section_iter(src_file, comrak_opts)?;
            html_sections.push(html_section);
        }

        let html = format!(
            "{}\n{}",
            src_file_html,
            html_sections
                .iter()
                .map(|html| format!("<div class=\"section\">{}</div>", html.html))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(HTMLDataSection { html })
    }
}
