//! Error

/// Extension trait to convert a Result from epub-builder to an AST error
pub(crate) trait ResultExt<T> {
    /// Converts to a
    fn into_ast_result(self) -> Result<T, docz_ast::Error>;
}

impl<T> ResultExt<T> for epub_builder::Result<T> {
    fn into_ast_result(self) -> Result<T, docz_ast::Error> {
        self.map_err(|e| docz_ast::Error::new(e.to_string().as_str()))
    }
}
