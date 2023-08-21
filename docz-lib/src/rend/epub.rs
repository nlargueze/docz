//! EPUB renderer

mod template;

use std::{fs, path::PathBuf};

use crate::{
    cfg::Config,
    src::{SourceData, SourceFile},
};

use self::template::{
    CHAPTER_TEMPLATE, CHAPTER_TEMPLATE_ID, COVER_TEMPLATE, COVER_TEMPLATE_ID, EPUB_STYLE_CSS,
    FONT_NOTO_SERIF_REGULAR,
};

use super::{comrak_options, markdown_to_html, Renderer};
use anyhow::{anyhow, bail, Context, Result};
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use handlebars::Handlebars;
use log::trace;
use serde::{Deserialize, Serialize};
use slug::slugify;

/// EPUB output config (from doc.toml)
#[derive(Debug, Deserialize, Default)]
pub struct EPUBOutputConfig {
    /// Cover image (path inside the assets folder)
    pub cover_image: Option<PathBuf>,
}

/// Renderer for EPUB
#[derive(Debug, Default)]
pub struct EPUBRenderer {
    registry: Handlebars<'static>,
}

impl EPUBRenderer {
    /// Creates a new EPUB renderer
    pub fn new() -> Self {
        Self::default()
    }
}

/// EPUB data
#[derive(Debug, Serialize)]
struct EPUBData {
    title: String,
    summary: String,
    authors: Vec<String>,
    cover_image: Option<PathBuf>,
    /// Assets (src path, dst path, eg `assets/img.jpg`)
    assets: Vec<(PathBuf, PathBuf)>,
    sections: Vec<EPUBSection>,
}

/// EPUB section
#[derive(Debug, Serialize)]
struct EPUBSection {
    id: String,
    path: PathBuf,
    title: String,
    html: String,
    sections: Vec<EPUBSection>,
}

impl Renderer for EPUBRenderer {
    fn register(&mut self, _cfg: &Config) -> Result<()> {
        self.registry = Handlebars::new();
        self.registry
            .register_template_string(COVER_TEMPLATE_ID, COVER_TEMPLATE)?;
        self.registry
            .register_template_string(CHAPTER_TEMPLATE_ID, CHAPTER_TEMPLATE)?;
        Ok(())
    }

    fn render(&self, cfg: &Config, src_data: &SourceData) -> Result<()> {
        // process the source files
        let data = process_src(src_data, cfg)?;

        // Create a new EpubBuilder using the zip library
        let zip = ZipLibrary::new().into_any()?;
        let mut builder = EpubBuilder::new(zip).into_any()?;

        // metadata
        builder.epub_version(epub_builder::EpubVersion::V30);
        builder.set_title(&data.title);
        builder.set_description(vec![data.summary.clone()]);
        builder.set_authors(data.authors.clone());

        // resources
        builder
            .add_resource("fonts/NotoSerif.ttf", FONT_NOTO_SERIF_REGULAR, "font/ttf")
            .into_any()?;
        for (asset_src, asset_dst) in &data.assets {
            let file = fs::File::open(asset_src)?;
            let mime = match mime_guess::from_path(asset_src).first() {
                Some(mime) => mime,
                None => {
                    bail!("Failed to guess mime type for asset: {:?}", asset_src);
                }
            };
            trace!("Added resource: {} ({})", asset_dst.display(), mime);
            builder
                .add_resource(asset_dst, file, mime.to_string())
                .into_any()?;
        }

        // stylesheet
        builder.stylesheet(EPUB_STYLE_CSS).into_any()?;

        // cover
        if let Some(cover_img_path) = &data.cover_image {
            let mime_type = match cover_img_path
                .extension()
                .ok_or(anyhow!("Missing cover file extension"))?
                .to_str()
                .ok_or(anyhow!("Missing cover file extension"))?
            {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                _ => bail!("Invalid cover file extension"),
            };
            let file = fs::File::open(cover_img_path)?;
            builder
                .add_cover_image("cover.png", file, mime_type)
                .into_any()?;
        } else {
            let xhtml = self.registry.render(COVER_TEMPLATE_ID, &data)?;

            let cover_content = EpubContent::new("cover.xhtml", xhtml.as_bytes())
                // .title("Cover")
                .reftype(ReferenceType::Cover);
            builder.add_content(cover_content).into_any()?;
        }

        // TOC
        builder.inline_toc();

        // sections
        for section in &data.sections {
            self.render_section_iter(&mut builder, section, 1)?;
        }

        // write
        let build_dir = cfg.build_dir();
        let epub_file = build_dir.join("doc.epub");
        let mut buffer = Vec::<u8>::new();
        builder.generate(&mut buffer).into_any()?;
        fs::write(epub_file, buffer)?;

        Ok(())
    }
}

impl EPUBRenderer {
    /// Renders sections iteratively
    fn render_section_iter(
        &self,
        builder: &mut EpubBuilder<ZipLibrary>,
        section: &EPUBSection,
        level: i32,
    ) -> Result<()> {
        let xhtml = self.registry.render(CHAPTER_TEMPLATE_ID, section)?;

        let href = section.path.to_str().unwrap();
        let title = &section.title;
        let content = EpubContent::new(href, xhtml.as_bytes())
            .title(title)
            .level(level)
            .reftype(ReferenceType::Text);
        builder.add_content(content).into_any()?;

        for child in &section.sections {
            self.render_section_iter(builder, child, level + 1)?;
        }

        Ok(())
    }
}

/// Processes the source data
fn process_src(data: &SourceData, cfg: &Config) -> Result<EPUBData> {
    let epub_config = cfg.get_output_cfg::<EPUBOutputConfig>("epub")?.unwrap();

    let title = cfg.file().doc.title.clone();
    let summary = cfg.file().doc.summary.clone();
    let authors = cfg.file().doc.authors.clone();
    let cover_image = epub_config.cover_image.clone();

    let src_dir = cfg.src_dir();
    let mut assets = vec![];
    for asset_path in &data.assets {
        let asset_path_stripped = asset_path.strip_prefix(&src_dir)?;
        assets.push((asset_path.to_owned(), asset_path_stripped.to_owned()));
    }

    let mut chapters = vec![];
    for src_file in &data.files {
        let chapter = process_src_file(src_file, cfg)?;
        chapters.push(chapter);
    }

    Ok(EPUBData {
        title,
        summary,
        authors,
        cover_image,
        assets,
        sections: chapters,
    })
}

/// Processes a source file
fn process_src_file(src_file: &SourceFile, cfg: &Config) -> Result<EPUBSection> {
    let id = {
        let file_name = src_file
            .path
            .file_stem()
            .ok_or(anyhow!("Invalid src file name"))?
            .to_str()
            .ok_or(anyhow!("Invalid src file name"))?;
        slugify(file_name)
    };

    let src_dir = cfg.src_dir();
    let path = src_file
        .path
        .strip_prefix(src_dir)
        .context("Source file path is not within the source dir")?
        .with_file_name(&id)
        .with_extension("xhtml");

    let comrak_opts = comrak_options();
    let content_str = String::from_utf8(src_file.content.to_vec())?;
    let (html, metadata) = markdown_to_html(&content_str, &comrak_opts)?;
    let title = metadata.title.unwrap_or("Missing title".to_string());

    // sections
    let mut sections = vec![];
    for child in &src_file.children {
        let section = process_src_file(child, cfg)?;
        sections.push(section);
    }

    Ok(EPUBSection {
        id,
        path,
        title,
        html,
        sections,
    })
}

trait EyreResultExt<T> {
    fn into_any(self) -> Result<T>;
}

impl<T> EyreResultExt<T> for epub_builder::Result<T> {
    fn into_any(self) -> Result<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(anyhow!(err.root_cause().to_string())),
        }
    }
}
