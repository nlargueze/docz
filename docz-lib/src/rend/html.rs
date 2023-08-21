//! HTML renderer

mod templates;

use std::{
    ffi::OsStr,
    fs,
    io::BufWriter,
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

use self::templates::HTMLTemplate;

use super::Renderer;

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

/// HTML index.html data
#[derive(Debug, Serialize)]
struct HTMLDocData {
    /// Title
    title: String,
    /// Authors
    authors: Vec<String>,
    /// Summary
    summary: String,
    /// All pages
    pages: Vec<HTMLPageData>,
}

/// HTML page data
#[derive(Debug, Clone, Serialize)]
struct HTMLPageData {
    /// Page ID (slugified)
    id: String,
    /// URL path
    path: PathBuf,
    /// Title
    title: String,
    /// Index (eg 1.2.4) - used for the table of contents
    index: String,
    /// HTML content
    html: String,
    /// Subpages pages
    pages: Vec<HTMLPageData>,
}

/// HTML page data
#[derive(Debug, Serialize)]
struct HTMLPageTemplateData<'a> {
    /// Page ID (slugified)
    id: String,
    /// URL path
    path: PathBuf,
    /// Title
    title: String,
    /// Index (eg 1.2.4) - used for the table of contents
    index: String,
    /// HTML content
    html: String,
    /// Subpages pages
    pages: Vec<HTMLPageData>,
    /// DOcument
    doc: &'a HTMLDocData,
}

/// File metadata
#[derive(Debug, Deserialize, Default)]
struct FileMetadata {
    title: Option<String>,
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
                    "article" => {
                        self.template = HTMLTemplate::article();
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
            "page",
            include_str!("html/templates/_partials/page_partial.hbs"),
        )?;
        self.registry.register_partial(
            "toc",
            include_str!("html/templates/_partials/toc_partial.hbs"),
        )?;

        Ok(())
    }

    fn render(&self, cfg: &Config, src_data: &SourceData) -> Result<()> {
        // process the source files to template data
        let doc = process_src_data(cfg, src_data)?;
        debug!("HTML template data \n{doc:#?}");

        // create HTML dir inside build (NB: /build has been cleared before)
        let build_dir = cfg.build_dir().join("html");
        fs::create_dir_all(&build_dir)?;

        // render index.html
        let index_file_str = self.registry.render(INDEX_TEMPLATE_ID, &doc)?;
        let index_file = build_dir.join("index.html");
        fs::write(index_file, index_file_str)?;

        // render {page}.html
        for page in &doc.pages {
            self.render_page_iter(page, &build_dir, &doc)?;
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
    /// Renders the individual pages
    fn render_page_iter(
        &self,
        page: &HTMLPageData,
        build_dir: &Path,
        doc: &HTMLDocData,
    ) -> Result<()> {
        let page = HTMLPageTemplateData {
            id: page.id.clone(),
            path: page.path.clone(),
            title: page.title.clone(),
            index: page.index.clone(),
            html: page.html.clone(),
            pages: page.pages.clone(),
            doc,
        };
        let page_file_str = self.registry.render(PAGE_TEMPLATE_ID, &page)?;
        let page_file = build_dir.join(&page.path);
        let parent_dir = page_file.parent().unwrap();
        fs::create_dir_all(parent_dir)?;
        fs::write(page_file, page_file_str)?;

        for page in &page.pages {
            self.render_page_iter(page, build_dir, doc)?;
        }
        Ok(())
    }
}

/// Returns the comrak options
fn comrak_options() -> ComrakOptions {
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
            github_pre_lang: false,
            full_info_string: true,
            width: 0,
            unsafe_: false,
            escape: false,
            list_style: comrak::ListStyleType::Dash,
            sourcepos: false,
        },
    }
}

/// Extracts the HTML data from the source data
fn process_src_data(cfg: &Config, src_data: &SourceData) -> Result<HTMLDocData> {
    let comrak_opts = comrak_options();
    let src_dir = cfg.src_dir();
    let pages = process_src_files_iter(&src_data.files, &comrak_opts, &src_dir, "")?;

    Ok(HTMLDocData {
        title: cfg.file().doc.title.to_string(),
        authors: cfg.file().doc.authors.to_vec(),
        summary: cfg.file().doc.summary.to_string(),
        pages,
    })
}

/// Processes source files recursively
fn process_src_files_iter(
    src_files: &[SourceFile],
    comrak_opts: &ComrakOptions,
    src_dir: &Path,
    parent_index: &str,
) -> Result<Vec<HTMLPageData>> {
    let mut pages = vec![];
    for (i, src_file) in src_files.iter().enumerate() {
        let index = format!(
            "{}{}{}",
            parent_index,
            if parent_index.is_empty() { "" } else { "." },
            i + 1
        );

        if let Some(page) = process_src_file_iter(src_file, comrak_opts, src_dir, &index)? {
            pages.push(page);
        };
    }
    Ok(pages)
}

/// Processes a source file recursively
fn process_src_file_iter(
    src_file: &SourceFile,
    comrak_opts: &ComrakOptions,
    src_dir: &Path,
    index: &str,
) -> Result<Option<HTMLPageData>> {
    let (html, metadata) = match src_file
        .path
        .extension()
        .map(|ext| ext.to_str().unwrap_or(""))
        .unwrap_or("")
    {
        "md" | "markdown" => {
            let content_str = String::from_utf8(src_file.content.to_vec())?;
            markdown_to_html(&content_str, comrak_opts)?
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

    let title = metadata.title.unwrap_or(id.clone());
    let index = index.to_string();

    let path = src_file
        .path
        .strip_prefix(src_dir)
        .context("Source file path is not within the source dir")?
        .with_file_name(&id)
        .with_extension("html");

    let pages = process_src_files_iter(&src_file.children, comrak_opts, src_dir, &index)?;

    Ok(Some(HTMLPageData {
        id,
        path,
        title,
        index,
        html,
        pages,
    }))
}

/// Extracts the markdown content and converts to HTML
fn markdown_to_html(md: &str, opts: &ComrakOptions) -> Result<(String, FileMetadata)> {
    // extract
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, md, opts);

    // frontmatter > metadata
    let metadata = match root.children().next() {
        Some(node) => {
            if let comrak::nodes::NodeValue::FrontMatter(ref fm) = node.data.borrow().value {
                let fm = match fm.strip_prefix("---") {
                    Some(fm) => fm.trim(),
                    None => return Err(anyhow!("Invalid frontmatter, missing leading ---")),
                };
                let fm = match fm.strip_suffix("---") {
                    Some(fm) => fm.trim(),
                    None => return Err(anyhow!("Invalid frontmatter, missing trailing --- )")),
                };
                serde_yaml::from_str::<FileMetadata>(fm)?
            } else {
                FileMetadata::default()
            }
        }
        None => FileMetadata::default(),
    };

    // > HTML
    let mut bw = BufWriter::new(Vec::new());
    comrak::format_html_with_plugins(root, opts, &mut bw, &comrak::ComrakPlugins::default())?;
    let html = String::from_utf8(bw.into_inner()?)?;

    Ok((html, metadata))
}
