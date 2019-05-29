extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

const API_TOC: &'static str = "https://www.gesetze-im-internet.de/gii-toc.xml";

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gesetz {
    pub title: String,
    pub link: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Toc {
    #[serde(rename = "item", default)]
    pub items: Vec<Gesetz>
}

impl Gesetz {
    pub fn new(title: &str, link: &str) -> Gesetz {
        Gesetz {
            title: title.into(),
            link: link.into(),
        }
    }
}

impl Toc {
    pub fn fetch_toc() -> Result<String, Box<::std::error::Error>> {
        reqwest::get(API_TOC)?.text().map_err(|e| e.into())
    }

    pub fn fetch() -> Result<Toc, Box<::std::error::Error>> {
        Self::fetch_toc()
            .and_then(|s| serde_xml_rs::from_str(&s).map_err(|e| e.into()))
    }
}

#[test]
fn test_can_create_gesetz() {
    let gesetz = Gesetz::new("Abgeordnetengesetz", "ABG");
    assert_eq!(gesetz.title, "Abgeordnetengesetz");
    assert_eq!(gesetz.link, "ABG");
}

#[test]
fn test_can_compare_gesetz() {
    let gesetz_a = Gesetz::new("A", "A");
    let gesetz_b = Gesetz::new("B", "A");
    let gesetz_c = Gesetz::new("A", "C");
    
    assert_ne!(gesetz_a, gesetz_b);
    assert_ne!(gesetz_a, gesetz_c);
    assert_ne!(gesetz_b, gesetz_c);
    
    assert_eq!(gesetz_a, gesetz_a);
    assert_eq!(gesetz_b, gesetz_b);
    assert_eq!(gesetz_c, gesetz_c);
}
