use futures::{Future, IntoFuture};
use log::{info, debug};
use reqwest::r#async::{Client as ReqwestClient, Response};
use std::fmt;
use url::Url;
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    UrlParseError(url::ParseError),
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

    pub fn get(&self, path: &str) -> impl Future<Item = Response, Error = Error> {
        let request_url = self.base_url.join(path);
        let client = self.reqwest.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| {
                info!("GET {}", url.as_str());

                client.get(url.as_str()).send().map_err(Error::from)
            })
            .map_err(Error::from)
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Extracts the first file from a Zip archive.
    fn extract_first_file(data: Vec<u8>) -> Result<String, zip::result::ZipError> {
        let reader = std::io::Cursor::new(data);
        let mut archive = zip::ZipArchive::new(reader)?;

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            debug!("Filename: {}", file.name());
            let first_byte = file.bytes().next().unwrap()?;
            debug!("{}", first_byte);
        }

        debug_assert!(archive.len() == 1);

        let mut file = archive.by_index(0).unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
