//! Rendering

use docz_ast::{Error, Node, Renderer};
use printpdf::{Mm, PdfDocument};

use crate::Pdf;

/// AST renderer for PDF
#[derive(Debug, Default)]
pub struct PDFRenderer {}

impl PDFRenderer {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer<Pdf> for PDFRenderer {
    fn render(&self, node: &Node<Pdf>) -> Result<Vec<u8>, Error> {
        // only a document can be rendered
        if let Pdf::Document {
            size,
            title,
            authors,
            summary,
        } = &node.data
        {
            // init doc
            let (size_w, size_h) = size
                .map(|(w, h)| (Mm(w), Mm(h)))
                .unwrap_or((Mm(210.0), Mm(297.0)));
            let (doc, page_1, layer_1) = PdfDocument::new(title, size_w, size_h, "Layer 1");
            let curr_layer = doc.get_page(page_1).get_layer(layer_1);

            // add fonts
            let font = doc.add_external_font_data(font_stream)
            let font = doc.get_font(font);

            // to bytes
            match doc.save_to_bytes() {
                Ok(bytes) => Ok(bytes),
                Err(err) => Err(Error::new(&err.to_string())),
            }
        } else {
            Err(Error::new("Only a document can be rendered"))
        }

        // // init document
        // let doc = self.render_iter(node, doc, page_1, layer_1)?;

        // doc.save_to_bytes().into_ast_result()
        // todo!()
    }
}

impl PDFRenderer {
    /// Renders the PDF document recursively
    fn render(&self, pdf: &Pdf) -> Result<PdfDocumentReference, Error> {
        // let curr_page = doc.get_page(page_idx);
        // let _curr_layer = curr_page.get_layer(layer_idx);
        // let _font = doc
        //     .add_builtin_font(BuiltinFont::Helvetica)
        //     .into_ast_result()?;

        // // curr_layer.begin_text_section();
        // // let text = "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.";
        // // curr_layer.use_text(text, 20.0, Mm(0.0), Mm(0.0), &font);
        // // curr_layer.end_text_section();
        // todo!("Render to PDF");

        // // if let Some(children) = node.children() {
        // //     for child in children {
        // //         self.render_iter(node, doc, page_idx, layer_idx)?;
        // //     }
        // // }

        // // Ok(doc)
        todo!()
    }
}
