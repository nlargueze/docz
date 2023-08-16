//! HTML renderer

use std::fs;

use anyhow::Result;
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions};
use fs_extra::dir::CopyOptions;
use handlebars::Handlebars;
use log::debug;
use serde::Serialize;

use crate::{
    cfg::Config,
    doc::{Document, Section},
};

use super::Renderer;

/// HTML template
const HTML_TEMPLATE_STR: &str = include_str!("templates/html.hbs");

/// Renderer for HTML docs
#[derive(Debug)]
pub struct HTMLRenderer {
    registry: Handlebars<'static>,
    template_id: String,
}

impl HTMLRenderer {
    /// Default template ID
    const DEFAULT_TEMPLATE_ID: &str = "_default_";

    /// Creates a new debug renderer
    pub fn new() -> Result<Self> {
        let mut registry = Handlebars::new();
        let template_id = Self::DEFAULT_TEMPLATE_ID.to_string();
        registry.register_template_string(&template_id, HTML_TEMPLATE_STR)?;
        Ok(Self {
            registry,
            template_id,
        })
    }

    /// Sets the template
    pub fn template(mut self, id: &str, template: &str) -> Result<Self> {
        self.registry.register_template_string(id, template)?;
        self.template_id = id.to_string();
        Ok(self)
    }
}

impl Renderer for HTMLRenderer {
    fn id(&self) -> &'static str {
        "html"
    }

    fn render(&self, cfg: &Config, doc: &crate::doc::Document) -> Result<()> {
        debug!("Renderer (html)");

        let data = self.extract_data(cfg, doc)?;
        debug!("{data:#?}");

        let out_data = self.registry.render(&self.template_id, &data)?;

        let build_dir = cfg.build_dir().join("html");
        fs::create_dir_all(&build_dir)?;

        let out_file = build_dir.join("index.html");
        fs::write(out_file, out_data)?;

        let src_assets_dir = cfg.assets_dir();
        fs_extra::copy_items(
            &[&src_assets_dir],
            build_dir.join("assets"),
            &CopyOptions::new().copy_inside(true),
        )?;

        Ok(())
    }
}

/// HTML data
#[derive(Debug, Default, Serialize)]
struct HTMLData {
    title: String,
    chapters: Vec<HTMLDataChapter>,
}

/// HTML chapter
#[derive(Debug, Default, Serialize)]
struct HTMLDataChapter {
    html: String,
}

impl HTMLRenderer {
    /// Extracts the data
    fn extract_data(&self, cfg: &Config, doc: &Document) -> Result<HTMLData> {
        let mut data = HTMLData {
            title: cfg.file().doc.title.to_string(),
            ..Default::default()
        };

        let comrak_opts = self.comrak_options();
        for section in &doc.sections {
            let html = self.extract_section_iter(section, &comrak_opts)?;
            let chapter = HTMLDataChapter { html };
            data.chapters.push(chapter);
        }

        Ok(data)
    }

    /// Sets the comrak options
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

    /// Renders a section recursively
    #[allow(clippy::only_used_in_recursion)]
    fn extract_section_iter(
        &self,
        section: &Section,
        comrak_opts: &ComrakOptions,
    ) -> Result<String> {
        let html_file = match section
            .file
            .extension()
            .map(|ext| ext.to_str().unwrap_or(""))
            .unwrap_or("")
        {
            "md" | "markdown" => {
                let content_str = String::from_utf8(section.content.to_vec())?;
                comrak::markdown_to_html(&content_str, comrak_opts)
            }
            _ => {
                debug!("Skipped file {}", section.file.display());
                String::new()
            }
        };

        let mut html_subsections = vec![];
        for subsection in &section.children {
            let subsec_html = self.extract_section_iter(subsection, comrak_opts)?;
            html_subsections.push(subsec_html);
        }

        let html = format!(
            "{}\n{}",
            html_file,
            html_subsections
                .iter()
                .map(|html| format!("<div class=\"section\">{html}</div>"))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(html)
    }
}
