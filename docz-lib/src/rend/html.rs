//! HTML renderer

use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions};
use fs_extra::dir::CopyOptions;
use handlebars::Handlebars;
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::{
    cfg::Config,
    src::{SourceData, SourceFile},
};

use super::Renderer;

/// index.html template ID
const INDEX_TEMPLATE_ID: &str = "_INDEX_";

/// {page}.html template ID
const PAGE_TEMPLATE_ID: &str = "_PAGE_";

/// HTML output config (from doc.toml)
#[derive(Debug, Deserialize, Default)]
pub struct OutputHtmlConfig {
    /// Builtin template ID
    pub template: Option<String>,
    /// Overwrites the `index.hbs` template
    pub index: Option<PathBuf>,
    /// Overwrites the `page.hbs` template
    pub page: Option<PathBuf>,
    /// Add static files to the build dir (or overwrite existing files)
    pub static_files: Option<Vec<(PathBuf, PathBuf)>>,
}

/// Renderer for HTML docs
#[derive(Debug, Default)]
pub struct HTMLRenderer {
    registry: Handlebars<'static>,
    template: HTMLTemplate,
}

impl HTMLRenderer {
    /// Creates a new HTML renderer
    pub fn new(template: HTMLTemplate) -> Self {
        Self {
            registry: Handlebars::default(),
            template,
        }
    }
}

/// HTML Template
#[derive(Debug)]
pub struct HTMLTemplate {
    /// ID
    pub id: &'static str,
    /// Handlebars template for index.html
    pub index: String,
    /// Handlebars template for {page}.html
    pub page: String,
    /// Embedded static files
    pub embed_static_files: Vec<(&'static str, &'static [u8])>,
    /// Static files to copy from the filesystem
    pub fs_static_files: Vec<(PathBuf, PathBuf)>,
}

impl Default for HTMLTemplate {
    fn default() -> Self {
        Self {
            id: "default",
            index: include_str!("html/tpl/default/index.hbs").to_string(),
            page: include_str!("html/tpl/default/page.hbs").to_string(),
            embed_static_files: vec![
                ("sse.js", include_bytes!("html/tpl/default/sse.js")),
                ("style.css", include_bytes!("html/tpl/default/style.css")),
                (
                    "favicon.png",
                    include_bytes!("html/tpl/default/favicon.png"),
                ),
                (
                    "favicon.svg",
                    include_bytes!("html/tpl/default/favicon.svg"),
                ),
            ],
            fs_static_files: vec![],
        }
    }
}

/// HTML index.html data
#[derive(Debug, Default, Serialize)]
struct HTMLIndexData {
    title: String,
    authors: Vec<String>,
    summary: String,
    pages: Vec<HTMLPageData>,
}

/// HTML page data
#[derive(Debug, Default, Serialize)]
struct HTMLPageData {
    /// Page ID (slugified)
    id: String,
    /// URL path
    path: PathBuf,
    /// HTML content
    html: String,
    /// Sub-pages
    pages: Vec<HTMLPageData>,
}

impl Renderer for HTMLRenderer {
    fn register(&mut self, cfg: &Config) -> Result<()> {
        // (re)init registry
        self.registry = Handlebars::new();

        // get config
        let html_config = cfg
            .get_output_cfg::<OutputHtmlConfig>("html")?
            .unwrap_or_default();

        // overwrite the default built-in template
        if let Some(cfg_template_id) = html_config.template {
            if cfg_template_id.as_str() != self.template.id {
                match cfg_template_id.as_str() {
                    "default" => {
                        self.template = HTMLTemplate::default();
                    }
                    _ => {
                        return Err(anyhow!("Unknown template ID: {}", cfg_template_id));
                    }
                }
            }
        }

        // overwrite the index template
        let root_dir = cfg.root_dir();
        if let Some(index_hbs_path) = html_config.index {
            let index_hbs_path = root_dir.join(index_hbs_path);
            trace!(
                "HTML template, overwriting index template by: {}",
                index_hbs_path.display()
            );
            if index_hbs_path.extension() != Some(OsStr::new("hbs")) {
                return Err(anyhow!(
                    "Invalid index template file: {}",
                    index_hbs_path.display()
                ));
            }
            let index_hbs_str = fs::read_to_string(&index_hbs_path).context(format!(
                "HTML index template file not found ({})",
                index_hbs_path.display()
            ))?;
            self.template.index = index_hbs_str;
        }

        // overwrite the page template
        if let Some(page_hbs_path) = html_config.page {
            let page_hbs_path = root_dir.join(page_hbs_path);
            trace!(
                "HTML template, overwriting page template by: {}",
                page_hbs_path.display()
            );
            if page_hbs_path.extension() != Some(OsStr::new("hbs")) {
                return Err(anyhow!(
                    "Invalid page template file: {}",
                    page_hbs_path.display()
                ));
            }
            let page_hbs_str = fs::read_to_string(&page_hbs_path).context(format!(
                "HTML page template file not found ({})",
                page_hbs_path.display()
            ))?;
            self.template.page = page_hbs_str;
        }

        // Add static assets to copy from the FS
        self.template.fs_static_files = html_config
            .static_files
            .unwrap_or_default()
            .into_iter()
            .map(|(src, dest)| (root_dir.join(src), dest))
            .collect();

        // register templates, partials, helpers with Handlebars
        self.registry
            .register_template_string(INDEX_TEMPLATE_ID, &self.template.index)?;
        self.registry
            .register_template_string(PAGE_TEMPLATE_ID, &self.template.page)?;
        self.registry.register_partial(
            "pagePartial",
            include_str!("html/tpl/default/page_partial.hbs"),
        )?;

        Ok(())
    }

    fn render(&self, cfg: &Config, src_data: &SourceData) -> Result<()> {
        // process the source files to template data
        let data = self.process_src_data(cfg, src_data)?;
        debug!("HTML template data \n{data:#?}");

        // create HTML dir inside build (NB: /build has been cleared before)
        let build_dir = cfg.build_dir().join("html");
        fs::create_dir_all(&build_dir)?;

        // render index.html
        let index_file_str = self.registry.render(INDEX_TEMPLATE_ID, &data)?;
        let index_file = build_dir.join("index.html");
        fs::write(index_file, index_file_str)?;

        // render {page}.html
        for page in &data.pages {
            self.render_page_iter(page, &build_dir)?;
        }

        // write embedded static files
        for (file_name, file_data) in &self.template.embed_static_files {
            fs::write(&build_dir.join(file_name), file_data)?;
        }

        // write filesystem static files
        for (src, dest) in &self.template.fs_static_files {
            fs::copy(src, &build_dir.join(dest))?;
        }

        // copy source assets
        let build_assets_dir = build_dir.join(cfg.assets_dir_name());
        fs::create_dir_all(&build_assets_dir)?;
        fs_extra::copy_items(&src_data.assets, build_assets_dir, &CopyOptions::new())?;

        Ok(())
    }
}

impl HTMLRenderer {
    /// Returns the comrak options
    fn comrak_options(&self) -> ComrakOptions {
        ComrakOptions {
            extension: ComrakExtensionOptions {
                strikethrough: true,
                tagfilter: false,
                table: true,
                autolink: true,
                tasklist: true,
                superscript: true,
                header_ids: None,
                footnotes: true,
                description_lists: true,
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
    fn process_src_data(&self, cfg: &Config, src_data: &SourceData) -> Result<HTMLIndexData> {
        let mut html_data = HTMLIndexData {
            title: cfg.file().doc.title.to_string(),
            authors: cfg.file().doc.authors.to_vec(),
            summary: cfg.file().doc.summary.to_string(),
            pages: vec![],
        };

        let comrak_opts = self.comrak_options();
        let src_dir = cfg.src_dir();
        for src_file in &src_data.files {
            if let Some(page) = self.process_src_file_data_iter(src_file, &comrak_opts, &src_dir)? {
                html_data.pages.push(page);
            };
        }

        Ok(html_data)
    }

    /// Renders a page recursively
    #[allow(clippy::only_used_in_recursion)]
    fn process_src_file_data_iter(
        &self,
        src_file: &SourceFile,
        comrak_opts: &ComrakOptions,
        src_dir: &Path,
    ) -> Result<Option<HTMLPageData>> {
        let html = match src_file
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
                warn!("Skipped file {}", src_file.path.display());
                return Ok(None);
            }
        };

        let id = {
            let file_name = src_file
                .path
                .file_stem()
                .ok_or(anyhow!("Invalid src file name"))?
                .to_str()
                .ok_or(anyhow!("Invalid src file name"))?;
            slugify(file_name)
        };

        let path = src_file
            .path
            .strip_prefix(src_dir)
            .context("Source file path is not within the source dir")?
            .with_file_name(&id)
            .with_extension("html");

        let mut pages = vec![];
        for src_file_page in &src_file.children {
            if let Some(page) =
                self.process_src_file_data_iter(src_file_page, comrak_opts, src_dir)?
            {
                pages.push(page);
            };
        }

        Ok(Some(HTMLPageData {
            id,
            path,
            html,
            pages,
        }))
    }

    /// Renders the individual pages
    fn render_page_iter(&self, page: &HTMLPageData, build_dir: &Path) -> Result<()> {
        let page_file_str = self.registry.render(PAGE_TEMPLATE_ID, &page)?;
        let page_file = build_dir.join(&page.path);
        let parent_dir = page_file.parent().unwrap();
        fs::create_dir_all(parent_dir)?;
        fs::write(page_file, page_file_str)?;

        for page in &page.pages {
            self.render_page_iter(page, build_dir)?;
        }
        Ok(())
    }
}
