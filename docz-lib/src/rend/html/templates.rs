//! HTML templates

use std::path::PathBuf;

/// HTML Template
#[derive(Debug)]
pub struct HTMLTemplate {
    /// ID
    pub id: &'static str,
    /// Handlebars template for index.html
    pub index: String,
    /// Handlebars template for {page}.html
    pub page: String,
    /// Embedded static files
    pub embed_static_files: Vec<(&'static str, &'static [u8])>,
    /// Static files to copy from the filesystem
    ///
    /// (source - relative to root, destination)
    pub fs_static_files: Vec<(PathBuf, PathBuf)>,
}

/// Shared static files
static SHARED_STATIC_FILES: &[(&str, &[u8])] = &[
    ("sse.js", include_bytes!("templates/_shared/sse.js")),
    (
        "favicon.png",
        include_bytes!("templates/_shared/favicon.png"),
    ),
    (
        "favicon.svg",
        include_bytes!("templates/_shared/favicon.svg"),
    ),
    (
        "prism-theme.css",
        include_bytes!("templates/_shared/prism-theme.css"),
    ),
    ("theme.js", include_bytes!("templates/_shared/theme.js")),
    ("sidebar.js", include_bytes!("templates/_shared/sidebar.js")),
];

impl Default for HTMLTemplate {
    fn default() -> Self {
        let mut embed_static_files: Vec<(&'static str, &'static [u8])> =
            vec![("style.css", include_bytes!("templates/default/style.css"))];
        embed_static_files.extend_from_slice(SHARED_STATIC_FILES);

        Self {
            id: "default",
            index: include_str!("templates/default/index.hbs").to_string(),
            page: include_str!("templates/default/page.hbs").to_string(),
            embed_static_files,
            fs_static_files: vec![],
        }
    }
}

impl HTMLTemplate {
    /// Returns the template for article
    pub fn article() -> Self {
        let mut embed_static_files: Vec<(&'static str, &'static [u8])> =
            vec![("style.css", include_bytes!("templates/article/style.css"))];
        embed_static_files.extend_from_slice(SHARED_STATIC_FILES);

        Self {
            id: "default",
            index: include_str!("templates/article/index.hbs").to_string(),
            page: include_str!("templates/article/page.hbs").to_string(),
            embed_static_files,
            fs_static_files: vec![],
        }
    }
}
