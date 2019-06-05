use futures::{Future, IntoFuture};
use log::info;
use url::Url;
use reqwest::r#async::{Client as ReqwestClient, Response};
use std::fmt;

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

impl std::error::Error for Error {
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    reqwest: ReqwestClient,
}

impl Client {
    pub fn new(
        base_url: Url,
        reqwest: ReqwestClient,
    ) -> Self {
        Client {
            base_url,
            reqwest,
        }
    }

    pub fn get(&self, path: &str) -> impl Future<Item = Response, Error = Error> {
        let request_url = self.base_url.join(path);
        let client = self.reqwest.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| {
                info!("GET {}", url.as_str());

                client
                    .get(url.as_str())
                    .send()
                    .map_err(Error::from)
            })
            .map_err(Error::from)
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}
