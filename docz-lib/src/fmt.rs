//! Formats

use std::{fmt::Display, path::Path, str::FromStr};

use anyhow::{anyhow, Error};

#[cfg(feature = "epub")]
pub mod epub;
#[cfg(feature = "html")]
pub mod html;
#[cfg(feature = "mkd")]
pub mod mkd;

// File format
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
pub enum Format {
    Markdown,
    Html,
    Pdf,
    Epub,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Format::Markdown => "md",
            Format::Html => "html",
            Format::Pdf => "pdf",
            Format::Epub => "epub",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md" => Ok(Format::Markdown),
            "html" => Ok(Format::Html),
            "pdf" => Ok(Format::Pdf),
            "epub" => Ok(Format::Epub),
            _ => Err(anyhow!("invalid format: {}", s)),
        }
    }
}

/// Extension trait to retrieve
pub trait FileExt {
    /// Retrieves the format based on the file extension
    fn format(&self) -> Option<Format>;
}

impl<'a> FileExt for &'a Path {
    fn format(&self) -> Option<Format> {
        match self.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => match ext {
                    "md" | "mdx" | "markdown" => Some(Format::Markdown),
                    "html" => Some(Format::Html),
                    _ => None,
                },
                None => None,
            },
            None => None,
        }
    }
}
