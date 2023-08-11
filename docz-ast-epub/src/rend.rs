//! Renderer

use docz_ast::{Error, Node, Renderer};
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};

use crate::error::ResultExt;

/// AST renderer for EPUB
#[derive(Debug, Default)]
pub struct EPubRenderer {
    /// Stylesheet
    pub stylesheet: String,
    /// Title
    pub title: String,
    /// Authors
    pub authors: Vec<String>,
    /// Cover image (name, MIME type, content)
    pub cover: Option<(String, String, Vec<u8>)>,
}

impl EPubRenderer {
    /// Creates a new instance
    pub fn new(title: &str) -> Self {
        Self {
            stylesheet: "".to_string(),
            title: title.to_string(),
            authors: vec![],
            cover: None,
        }
    }

    /// Assigns a stylesheet
    pub fn stylesheet(mut self, stylesheet: &str) -> Self {
        self.stylesheet = stylesheet.to_string();
        self
    }

    /// Adds an author
    pub fn author(mut self, author: &str) -> Self {
        self.authors.push(author.to_string());
        self
    }

    /// Assigns a cover image
    pub fn cover(mut self, name: &str, mime_type: &str, data: Vec<u8>) -> Self {
        self.cover = Some((name.to_string(), mime_type.to_string(), data));
        self
    }
}

impl Renderer for EPubRenderer {
    fn render(&self, node: &Node) -> Result<Vec<u8>, Error> {
        let zip = ZipLibrary::new().into_ast_result()?;
        let mut builder = EpubBuilder::new(zip).into_ast_result()?;

        // stylesheet
        builder
            .stylesheet(self.stylesheet.as_bytes())
            .into_ast_result()?;

        // metadata
        builder
            .metadata("title", self.title.clone())
            .into_ast_result()?;
        for authr in &self.authors {
            builder
                .metadata("author", authr.clone())
                .into_ast_result()?;
        }

        // cover image
        if let Some(cover) = &self.cover {
            let name = &cover.0;
            let mime_type = &cover.1;
            let data = &cover.2 as &[u8];
            builder
                .add_cover_image(name, data, mime_type)
                .into_ast_result()?;
        }

        // cover page
        let cover_content = EpubContent::new("cover.xhtml", "<div>Cover</div>".as_bytes())
            .title("Cover")
            .reftype(ReferenceType::Cover);
        builder.add_content(cover_content).into_ast_result()?;

        // render the nodes
        self.render_iter(node, &mut builder)?;

        // TOC
        builder.inline_toc();

        let mut buffer: Vec<u8> = vec![];
        builder.generate(&mut buffer).into_ast_result()?;
        Ok(buffer)
    }
}

impl EPubRenderer {
    /// Renders the PDF document recursively
    fn render_iter(
        &self,
        _node: &Node,
        _builder: &mut EpubBuilder<ZipLibrary>,
    ) -> Result<(), Error> {
        // // Add a image cover file
        //     .add_cover_image("cover.png", dummy_image.as_bytes(), "image/png")?
        // // Add a resource that is not part of the linear document structure
        //     .add_resource("some_image.png", dummy_image.as_bytes(), "image/png")?

        // // Add a title page
        //     .add_content(EpubContent::new("title.xhtml", dummy_content.as_bytes())
        //                  .title("Title")
        //                  .reftype(ReferenceType:: ))?
        // // Add a chapter, mark it as beginning of the "real content"
        //     .add_content(EpubContent::new("chapter_1.xhtml", dummy_content.as_bytes())
        //                  .title("Chapter 1")
        //                  .reftype(ReferenceType::Text))?
        // // Add a second chapter; this one has more toc information about its internal structure
        //     .add_content(EpubContent::new("chapter_2.xhtml", dummy_content.as_bytes())
        //                  .title("Chapter 2")
        //                  .child(TocElement::new("chapter_2.xhtml#1", "Chapter 2, section 1")))?
        // // Add a section. Since its level is set to 2, it will be attached to the previous chapter.
        //     .add_content(EpubContent::new("section.xhtml", dummy_content.as_bytes())
        //                  .title("Chapter 2, section 2")
        //                  .level(2))?
        // // Add a chapter without a title, which will thus not appear in the TOC.
        //     .add_content(EpubContent::new("notes.xhtml", dummy_content.as_bytes()))?

        // curr_layer.begin_text_section();
        // let text = "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.";
        // curr_layer.use_text(text, 20.0, Mm(0.0), Mm(0.0), &font);
        // curr_layer.end_text_section();
        // todo!("Render to EPUB");

        // if let Some(children) = node.children() {
        //     for child in children {
        //         self.render_iter(node, doc, page_idx, layer_idx)?;
        //     }
        // }

        Ok(())
    }
}
