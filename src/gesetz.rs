extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;
extern crate zip;

use serde::{Deserialize, Serialize};
use std::io::prelude::*;

/// API endpoint to get current table of contents.
const API_TOC: &'static str = "https://www.gesetze-im-internet.de/gii-toc.xml";

/// Entry in the table of content of current laws.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TocItem {
    /// The title of the law.
    pub title: String,
    /// Link to the XML file with the contents of the law.
    pub link: String,
}

/// Table of content of current laws.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Toc {
    /// List of items.
    #[serde(rename = "item", default)]
    pub items: Vec<TocItem>
}

impl TocItem {
    /// Create new TocItem with given properties.
    pub fn new(title: &str, link: &str) -> Self {
        Self {
            title: title.into(),
            link: link.into(),
        }
    }

    /// Fetch this law.
    pub fn fetch(&self) -> Result<String, Box<::std::error::Error>> {
        let mut response = reqwest::get(&self.link)?;
        let mut body = Vec::new();
        response.read_to_end(&mut body)?;
        let reader = std::io::Cursor::new(body);
        let mut archive = zip::ZipArchive::new(reader)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            println!("Filename: {}", file.name());
            let first_byte = file.bytes().next().unwrap()?;
            println!("{}", first_byte);
        }

        Ok(String::new())
    }
}

impl Toc {
    /// Fetch the current table of contents from the server.
    pub fn fetch_toc() -> Result<String, Box<::std::error::Error>> {
        reqwest::get(API_TOC)?.text().map_err(|e| e.into())
    }

    /// Fetch the current table of contents from the server and parse it, yielding a Toc.
    pub fn fetch() -> Result<Self, Box<::std::error::Error>> {
        Self::fetch_toc()
            .and_then(|s| serde_xml_rs::from_str(&s).map_err(|e| e.into()))
    }
}

#[test]
fn test_can_create_toc_item() {
    let gesetz = TocItem::new("Abgeordnetengesetz", "ABG");
    assert_eq!(gesetz.title, "Abgeordnetengesetz");
    assert_eq!(gesetz.link, "ABG");
}

#[test]
fn test_can_compare_toc_item() {
    let gesetz_a = TocItem::new("A", "A");
    let gesetz_b = TocItem::new("B", "A");
    let gesetz_c = TocItem::new("A", "C");
    
    assert_ne!(gesetz_a, gesetz_b);
    assert_ne!(gesetz_a, gesetz_c);
    assert_ne!(gesetz_b, gesetz_c);
    
    assert_eq!(gesetz_a, gesetz_a);
    assert_eq!(gesetz_b, gesetz_b);
    assert_eq!(gesetz_c, gesetz_c);
}