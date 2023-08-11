//! Error

/// Extension trait to convert a Result from genpdf to an AST error
pub(crate) trait ResultExt<T> {
    /// Converts to a
    fn into_ast_result(self) -> Result<T, docz_ast::Error>;
}

impl<T> ResultExt<T> for Result<T, printpdf::Error> {
    fn into_ast_result(self) -> Result<T, docz_ast::Error> {
        self.map_err(|e| docz_ast::Error::new(e.to_string().as_str()))
    }
}
