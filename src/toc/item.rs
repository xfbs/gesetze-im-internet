use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use url::{Url, ParseError};

/// Entry in the table of content of current laws.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TocItem {
    /// The title of the law.
    pub title: String,
    /// Link to the XML file with the contents of the law.
    pub link: String,
}

impl TocItem {
    /// Create new TocItem with given properties.
    pub fn new(title: String, link: String) -> Self {
        Self { title, link }
    }

    /// Extract law abbreviation from URL.
    pub fn short(&self) -> Option<&str> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^http://www.gesetze-im-internet.de/(.+)/xml.zip$").unwrap();
        }
        REGEX
            .captures(&self.link)
            .and_then(|c| c.get(1).map(|s| s.as_str()))
    }

    /// Parses URL of this TocItem.
    pub fn url(&self) -> Result<Url, ParseError> {
        Url::parse(&self.link)
    }
}

#[test]
fn test_can_create_toc_item() {
    let gesetz = TocItem::new("Abgeordnetengesetz".into(), "ABG".into());
    assert_eq!(gesetz.title, "Abgeordnetengesetz");
    assert_eq!(gesetz.link, "ABG");
}

#[test]
fn test_can_compare_toc_item() {
    let gesetz_a = TocItem::new("A".into(), "A".into());
    let gesetz_b = TocItem::new("B".into(), "A".into());
    let gesetz_c = TocItem::new("A".into(), "C".into());

    assert_ne!(gesetz_a, gesetz_b);
    assert_ne!(gesetz_a, gesetz_c);
    assert_ne!(gesetz_b, gesetz_c);

    assert_eq!(gesetz_a, gesetz_a);
    assert_eq!(gesetz_b, gesetz_b);
    assert_eq!(gesetz_c, gesetz_c);
}

#[test]
fn test_can_parse_short() {
    let item = TocItem::new(
        "Gesetz".into(),
        "http://www.gesetze-im-internet.de/1-dm-goldm_nzg/xml.zip".into(),
    );
    assert_eq!(item.short(), Some("1-dm-goldm_nzg"));
}

#[test]
fn test_can_parse_url() {
    let item = TocItem::new(
        "Gesetz".into(),
        "http://www.gesetze-im-internet.de/1-dm-goldm_nzg/xml.zip".into(),
    );
    let url = item.url().unwrap();
    assert_eq!(url.host_str(), Some("www.gesetze-im-internet.de"));
} 
