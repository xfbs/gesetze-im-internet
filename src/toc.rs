use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use url::{ParseError, Url};

mod item;
pub use item::TocItem;

/// Table of content of current laws.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Toc {
    /// List of items.
    #[serde(rename = "item", default)]
    pub items: Vec<TocItem>,
}

impl Toc {
    /// Load table of contents from string.
    pub fn from_str(input: &str) -> Result<Self, serde_xml_rs::Error> {
        serde_xml_rs::from_str(input)
    }

    /// Load table of contents from reader.
    pub fn from_reader<'de, R: Read, T: Deserialize<'de>>(
        reader: R,
    ) -> Result<Self, serde_xml_rs::Error> {
        serde_xml_rs::from_reader(reader)
    }

    /// Create new empty table of contents
    pub fn new(items: Vec<TocItem>) -> Self {
        Toc { items }
    }
}

impl Default for Toc {
    fn default() -> Self {
        Toc { items: Vec::new() }
    }
}
