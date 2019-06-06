use futures::{Future, IntoFuture, Stream};
use log::{debug, info};
use reqwest::r#async::{Client as ReqwestClient, Decoder, Response};
use std::fmt;
use std::io::Read;
use url::Url;

/// API endpoint to get current table of contents.
const API_TOC: &'static str = "https://www.gesetze-im-internet.de/gii-toc.xml";

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    UrlParseError(url::ParseError),
    ZipError(zip::result::ZipError),
    IOError(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::ReqwestError(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Error {
        Error::UrlParseError(e)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Error {
        Error::ZipError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    reqwest: ReqwestClient,
}

impl Client {
    pub fn new(base_url: Url, reqwest: ReqwestClient) -> Self {
        Client { base_url, reqwest }
    }

    fn get(&self, url: Url) -> impl Future<Item = Response, Error = Error> {
        info!("GET {}", url.as_str());
        self.reqwest.get(url.as_str()).send().map_err(Error::from)
    }

    pub fn get_toc(&self, path: &str) -> impl Future<Item = String, Error = Error> {
        let request_url = self.base_url.join(path);
        let me = self.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| me.get(url))
            .and_then(Self::read_data)
            .and_then(Self::extract_first_file)
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

    fn read_data(res: Response) -> impl Future<Item = Vec<u8>, Error = Error> {
        let mut res = res;
        let body = std::mem::replace(res.body_mut(), Decoder::empty());
        body.concat2()
            .map(|s| s.into_iter().collect())
            .map_err(Error::from)
    }
}
