//! EPUB

use std::{collections::HashMap, path::PathBuf};

use crate::{
    conv::{Exporter, Extractor},
    doc::Fragment,
};

use anyhow::{anyhow, Result};
use epub_builder::{EpubBuilder, ZipLibrary};

/// EPUB extractor
#[derive(Debug)]
pub struct EpubExtractor {}

impl Extractor for EpubExtractor {
    fn extract(&self, _data: &[u8]) -> Result<Fragment> {
        // let buf = &mut data;
        // let doc = EpubDoc::from_reader(data)?;

        todo!()
    }
}

/// EPUB exporter
#[derive(Debug)]
pub struct EpubExporter {}

impl Exporter for EpubExporter {
    fn export(&self, _doc: &crate::doc::Document) -> Result<HashMap<PathBuf, Vec<u8>>> {
        let zip = ZipLibrary::new().map_err(|err| anyhow!(err))?;
        let mut builder = EpubBuilder::new(zip).map_err(|err| anyhow!(err))?;

        let _x = builder
            .metadata("title", "abcd")
            .map_err(|err| anyhow!(err))?;
        todo!()
    }
}
