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
    src::{SourceData, SourceFile},
};

use super::Renderer;

/// SSE JS file
pub const SSE_JS: &[u8] = include_bytes!("html/sse.js");

/// Renderer for HTML docs
#[derive(Debug)]
pub struct HTMLRenderer {
    registry: Handlebars<'static>,
    template: &'static HTMLTemplate,
}

/// HTML Template
#[derive(Debug)]
pub struct HTMLTemplate {
    /// Template ID
    pub id: &'static str,
    /// Template file
    pub file: &'static str,
    /// Static files
    pub static_files: &'static [(&'static str, &'static [u8])],
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

/// Default template
static DEFAULT_TEMPLATE: HTMLTemplate = HTMLTemplate {
    id: "_default_",
    file: include_str!("html/index.hbs"),
    static_files: &[("sse.js", SSE_JS)],
};

impl HTMLRenderer {
    /// Creates a new HTML renderer
    pub fn new() -> Result<Self> {
        let mut registry = Handlebars::new();
        let template = &DEFAULT_TEMPLATE;
        registry.register_template_string(template.id, template.file)?;
        Ok(Self { registry, template })
    }

    /// Sets the template to use
    pub fn template(mut self, id: &str, template: &'static HTMLTemplate) -> Result<Self> {
        self.template = template;
        self.registry.register_template_string(id, template.file)?;
        Ok(self)
    }
}

impl Renderer for HTMLRenderer {
    fn render(&self, cfg: &Config, data: &SourceData) -> Result<()> {
        debug!("Renderer (html)");

        let html_data = self.extract_html_data(cfg, data)?;
        debug!("{html_data:#?}");
        let index_file_bytes = self.registry.render(self.template.id, &html_data)?;

        let build_dir = cfg.build_dir().join("html");
        fs::create_dir_all(&build_dir)?;

        let index_file = build_dir.join("index.html");
        fs::write(index_file, index_file_bytes)?;

        for (file_name, file_data) in self.template.static_files {
            let file = build_dir.join(file_name);
            fs::write(&file, file_data)?;
        }

        let src_assets_dir = cfg.assets_dir();
        fs_extra::copy_items(
            &[&src_assets_dir],
            build_dir.join("assets"),
            &CopyOptions::new().copy_inside(true),
        )?;

        debug!("Renderer (html) - OK");
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
    fn extract_html_data(&self, cfg: &Config, data: &SourceData) -> Result<HTMLData> {
        let mut html_data = HTMLData {
            title: cfg.file().doc.title.to_string(),
            sections: vec![],
        };

        let comrak_opts = self.comrak_options();
        for src_file in &data.files {
            let html_section = self.extract_html_section_iter(src_file, &comrak_opts)?;
            html_data.sections.push(html_section);
        }

        Ok(html_data)
    }

    /// Renders a section recursively
    #[allow(clippy::only_used_in_recursion)]
    fn extract_html_section_iter(
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
            let html_section = self.extract_html_section_iter(src_file, comrak_opts)?;
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
