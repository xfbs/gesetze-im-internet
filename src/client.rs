use crate::Toc;
use futures::{Future, IntoFuture, Stream};
use log::{debug, info};
use reqwest::r#async::{Client as ReqwestClient, Decoder, Response};
use std::fmt;
use std::io::Read;
use url::Url;
use quick_error::quick_error;

/// API endpoint to get current table of contents.
const API_TOC: &'static str = "https://www.gesetze-im-internet.de/gii-toc.xml";

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ReqwestError(err: reqwest::Error) {
            from()
        }
        UrlParseError(err: url::ParseError) {
            from()
        }
        ZipError(err: zip::result::ZipError) {
            from()
        }
        IOError(err: std::io::Error) {
            from()
        }
        ParseError(err: serde_xml_rs::Error) {
            from()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    reqwest: ReqwestClient,
}

/// Client
///
/// This is a Client used to connect to the GesetzeImInternet server and query things from it.
impl Client {
    pub fn new(base_url: Url, reqwest: ReqwestClient) -> Self {
        Client { base_url, reqwest }
    }

    /// Creates a request for a given URL.
    fn get(&self, url: Url) -> impl Future<Item = Response, Error = Error> {
        info!("GET {}", url.as_str());
        self.reqwest.get(url.as_str()).send().map_err(Error::from)
    }

    /// Retrieve the table of contents.
    pub fn get_toc(&self, path: &str) -> impl Future<Item = Toc, Error = Error> {
        let request_url = self.base_url.join(path);
        let me = self.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| me.get(url))
            .and_then(Self::read_data)
            .and_then(Self::extract_first_file)
            .and_then(Self::parse_toc)
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Extracts the first file from a Zip archive.
    fn extract_first_file(data: Vec<u8>) -> Result<String, Error> {
        let reader = std::io::Cursor::new(data);
        let mut archive = zip::ZipArchive::new(reader)?;

        debug_assert!(archive.len() == 1);

        let mut file = archive.by_index(0).unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Reads data from a response into a Vec<u8> (as a future).
    fn read_data(res: Response) -> impl Future<Item = Vec<u8>, Error = Error> {
        let mut res = res;
        let body = std::mem::replace(res.body_mut(), Decoder::empty());
        body.concat2()
            .map(|s| s.into_iter().collect())
            .map_err(Error::from)
    }

    /// Parse a string into a Toc.
    fn parse_toc(s: String) -> Result<Toc, Error> {
        Toc::from_str(&s)
            .map_err(Error::from)
    }
}
