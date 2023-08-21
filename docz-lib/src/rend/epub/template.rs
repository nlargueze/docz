//! EPUB template

/// EPUB style.css
pub static EPUB_STYLE_CSS: &[u8] = include_bytes!("template/style.css");

/// Font Noto regular
pub static FONT_NOTO_SERIF_REGULAR: &[u8] =
    include_bytes!("template/fonts/NotoSerif/NotoSerif-Regular.ttf");

/// Cover template ID
pub const COVER_TEMPLATE_ID: &str = "_cover_";

/// Cover template ID
pub static COVER_TEMPLATE: &str = include_str!("template/cover.hbs");

/// Chapter template ID
pub const CHAPTER_TEMPLATE_ID: &str = "_chapter_";

/// Chapter template ID
pub static CHAPTER_TEMPLATE: &str = include_str!("template/chapter.hbs");
