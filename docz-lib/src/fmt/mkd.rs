//! Markdown

use anyhow::Result;
use log::debug;
use pulldown_cmark::{Event, Parser, Tag};

use crate::{
    conv::{Exporter, Extractor},
    doc::Fragment,
};

/// Markdown extractor
#[derive(Debug, Default)]
pub struct MarkdownExtractor {}

impl Extractor for MarkdownExtractor {
    fn extract(&self, data: &[u8]) -> Result<Fragment> {
        let data_str = std::str::from_utf8(data)?;
        let parser = Parser::new(data_str);
        for event in parser {
            debug!("{:#?}", event);
            match event {
                Event::Start(tag) => match tag {
                    Tag::Paragraph => todo!(),
                    Tag::Heading(_, _, _) => todo!(),
                    Tag::BlockQuote => todo!(),
                    Tag::CodeBlock(_) => todo!(),
                    Tag::List(_) => todo!(),
                    Tag::Item => todo!(),
                    Tag::FootnoteDefinition(_) => todo!(),
                    Tag::Table(_) => todo!(),
                    Tag::TableHead => todo!(),
                    Tag::TableRow => todo!(),
                    Tag::TableCell => todo!(),
                    Tag::Emphasis => todo!(),
                    Tag::Strong => todo!(),
                    Tag::Strikethrough => todo!(),
                    Tag::Link(_, _, _) => todo!(),
                    Tag::Image(_, _, _) => todo!(),
                },
                Event::End(_tag) => todo!(),
                Event::Text(_text) => todo!(),
                Event::Code(_text) => todo!(),
                Event::Html(_text) => todo!(),
                Event::FootnoteReference(_text) => todo!(),
                Event::SoftBreak => todo!(),
                Event::HardBreak => todo!(),
                Event::Rule => todo!(),
                Event::TaskListMarker(_checked) => todo!(),
            }
        }
        todo!();
    }
}

/// Markdown exporter
#[derive(Debug, Default)]
pub struct MarkdownExporter {}

impl Exporter for MarkdownExporter {
    fn export(
        &self,
        _doc: &crate::doc::Document,
    ) -> Result<std::collections::HashMap<std::path::PathBuf, Vec<u8>>> {
        todo!()
    }
}
