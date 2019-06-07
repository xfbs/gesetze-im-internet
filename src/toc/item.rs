use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

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
