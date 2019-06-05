extern crate serde;
extern crate serde_xml_rs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gesetz {
}
