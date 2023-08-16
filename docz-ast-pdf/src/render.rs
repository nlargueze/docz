//! Rendering

use printpdf::{
    IndirectFontRef, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex,
};

use crate::{fonts::POPPINS_REGULAR, Error, PdfNode};

/// Render Options
#[derive(Debug, Clone, Default)]
pub struct RenderOptions {}

/// Internal buffer for rendering
struct RenderBuffer<'a> {
    doc: &'a PdfDocumentReference,
    curr_page: PdfPageIndex,
    curr_layer: PdfLayerIndex,
    font: &'a IndirectFontRef,
}

impl PdfNode {
    /// Renders the PDF document
    pub fn render(&self, _opts: RenderOptions) -> Result<Vec<u8>, Error> {
        let doc = match self {
            PdfNode::Document {
                size,
                title,
                authors: _,
                summary: _,
                children,
            } => {
                let (size_w, size_h) = size
                    .map(|(w, h)| (Mm(w), Mm(h)))
                    .unwrap_or((Mm(210.0), Mm(297.0)));
                let (doc, page_1, layer_1) = PdfDocument::new(title, size_w, size_h, "Layer 1");

                // fonts
                let font = doc.add_external_font(POPPINS_REGULAR)?;

                // 1st page
                let curr_layer = doc.get_page(page_1).get_layer(layer_1);
                curr_layer.use_text(title, 16.0, Mm(10.0), Mm(200.0), &font);

                // iter children
                let mut buffer = RenderBuffer {
                    doc: &doc,
                    curr_page: page_1,
                    curr_layer: layer_1,
                    font: &font,
                };

                for child in children {
                    render_node_iter(&mut buffer, child)?;
                }

                doc
            }
            _ => {
                return Err(Error::new("Only a document can be rendered"));
            }
        };

        Ok(doc.save_to_bytes()?)
    }
}

/// Renders a node recursively
fn render_node_iter(buffer: &mut RenderBuffer, node: &PdfNode) -> Result<(), Error> {
    let doc = buffer.doc;
    let curr_layer = doc.get_page(buffer.curr_page).get_layer(buffer.curr_layer);
    let default_font = buffer.font;

    match node {
        PdfNode::Document { .. } => unreachable!(),
        PdfNode::Section {
            level: _,
            index: _,
            title: _,
            children,
        } => {
            render_children(buffer, children)?;
        }
        PdfNode::Paragraph { children } => {
            render_children(buffer, children)?;
        }
        PdfNode::Text { value } => {
            curr_layer.begin_text_section();
            curr_layer.set_text_cursor(Mm(10.0), Mm(180.0));
            curr_layer.set_line_height(12.0);
            let parts = textwrap::wrap_columns(
                value,
                1,
                textwrap::Options::from(10).break_words(true),
                "",
                "",
                "",
            );
            for part in parts {
                curr_layer.write_text(part, default_font);
                curr_layer.add_line_break();
            }
            curr_layer.end_text_section();
        }
        PdfNode::Image { url: _ } => {
            todo!("Image rendering")
        }
        PdfNode::Formula {
            inline: _,
            value: _,
        } => {
            todo!("Formula rendering")
        }
    }

    Ok(())
}

/// Renders children
fn render_children(buffer: &mut RenderBuffer, children: &[PdfNode]) -> Result<(), Error> {
    for child in children {
        render_node_iter(buffer, child)?;
    }
    Ok(())
}
