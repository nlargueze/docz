//! Renderer

use docz_ast::{Error, Node, Renderer};
use printpdf::{BuiltinFont, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex};

use crate::error::ResultExt;

/// AST renderer for PDF
#[derive(Debug, Default)]
pub struct PDFRenderer {
    /// Custom size (defaults to A4 (210mm x 297mm))
    pub size: Option<(f64, f64)>,
}

impl PDFRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the page size
    fn page_size(&self) -> (Mm, Mm) {
        self.size
            .map(|(w, h)| (Mm(w), Mm(h)))
            .unwrap_or((Mm(210.0), Mm(297.0)))
    }
}

impl Renderer for PDFRenderer {
    fn is_binary(&self) -> bool {
        true
    }

    fn render(&self, node: &Node) -> Result<Vec<u8>, Error> {
        // init document
        let (size_w, size_h) = self.page_size();
        let (doc, page_1, layer_1) =
            PdfDocument::new("PDF_Document_title", size_w, size_h, "Layer 1");
        let doc = self.render_iter(node, doc, page_1, layer_1)?;

        doc.save_to_bytes().into_ast_result()
    }
}

impl PDFRenderer {
    /// Renders the PDF document recursively
    fn render_iter(
        &self,
        _node: &Node,
        doc: PdfDocumentReference,
        page_idx: PdfPageIndex,
        layer_idx: PdfLayerIndex,
    ) -> Result<PdfDocumentReference, Error> {
        let curr_page = doc.get_page(page_idx);
        let _curr_layer = curr_page.get_layer(layer_idx);
        let _font = doc
            .add_builtin_font(BuiltinFont::Helvetica)
            .into_ast_result()?;

        // curr_layer.begin_text_section();
        // let text = "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.";
        // curr_layer.use_text(text, 20.0, Mm(0.0), Mm(0.0), &font);
        // curr_layer.end_text_section();
        todo!("Render to PDF");

        // if let Some(children) = node.children() {
        //     for child in children {
        //         self.render_iter(node, doc, page_idx, layer_idx)?;
        //     }
        // }

        // Ok(doc)
    }
}
