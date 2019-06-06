use crate::Toc;
use error_chain::error_chain;
use futures::{Future, IntoFuture, Stream};
use lazy_static::lazy_static;
use log::{debug, info};
use reqwest::r#async::{Client as ReqwestClient, ClientBuilder, Decoder, Response};
use std::fmt;
use std::io::Read;
use url::Url;

/// Base url of Gesetze im Internet API.
const BASE_URL_STR: &'static str = "https://www.gesetze-im-internet.de";

/// Endpoint to get the table of contents.
const TOC_ENDPOINT: &'static str = "/gii-toc.xml";

lazy_static! {
    static ref BASE_URL: Url = Url::parse(BASE_URL_STR).unwrap();
}

error_chain! {
    foreign_links {
        ReqwestError(reqwest::Error);
        UrlParseError(url::ParseError);
        ZipError(zip::result::ZipError);
        IOError(std::io::Error);
        ParseError(serde_xml_rs::Error);
        DataParseError(std::string::FromUtf8Error);
    }
}

/// Client
///
/// This is a Client used to connect to the GesetzeImInternet server and query things from it.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    reqwest: ReqwestClient,
}

impl Client {
    /// Try to create new client.
    ///
    /// Might fail if no TLS implementation is found.
    pub fn new(base_url: Url) -> Result<Self> {
        let reqwest = ClientBuilder::new().build()?;
        Ok(Client { base_url, reqwest })
    }

    /// Creates a request for a given URL.
    fn get(&self, url: Url) -> impl Future<Item = Response, Error = Error> {
        info!("GET {}", url.as_str());
        self.reqwest.get(url.as_str()).send().map_err(Error::from)
    }

    /// Retrieve the table of contents.
    pub fn get_toc(&self) -> impl Future<Item = Toc, Error = Error> {
        let request_url = self.base_url.join(TOC_ENDPOINT);
        let me = self.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| me.get(url))
            .and_then(Self::read_data)
            .and_then(Self::data_to_string)
            .and_then(Self::parse_toc)
    }

    /// Get base url.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Extracts the first file from a Zip archive.
    fn extract_first_file(data: Vec<u8>) -> Result<String> {
        let reader = std::io::Cursor::new(data);
        let mut archive = zip::ZipArchive::new(reader)?;

        debug_assert!(archive.len() == 1);

        let mut file = archive.by_index(0)?;
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

    /// Reads data from an array of bytes into a string.
    ///
    /// Assumes the data is utf8 encoded. Might fail if illegal characters are encountered.
    fn data_to_string(data: Vec<u8>) -> Result<String> {
        String::from_utf8(data).map_err(Error::from)
    }

    /// Parse a string into a Toc.
    fn parse_toc(s: String) -> Result<Toc> {
        Toc::from_str(&s).map_err(Error::from)
    }
}

impl Default for Client {
    fn default() -> Self {
        Client::new(BASE_URL.clone()).unwrap()
    }
}
