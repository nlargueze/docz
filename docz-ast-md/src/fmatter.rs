//! FrontMatter

use docz_ast::Error;
use serde::Deserialize;

/// Front matter separator
pub const FRONTMATTER_SEP: &str = "---";

/// Parses the frontmatter YAML string
pub fn parse_frontmatter<'a, T>(value: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    serde_yaml::from_str::<T>(value).map_err(|e| Error::new(&e.to_string()))
}

/// Renders the frontmatter YAML string
pub fn serialize_frontmatter<T>(value: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    serde_yaml::to_string(value).map_err(|e| Error::new(&e.to_string()))
}
