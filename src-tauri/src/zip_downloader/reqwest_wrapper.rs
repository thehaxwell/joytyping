use bytes:: Bytes;
use reqwest::{self, Url, StatusCode};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ReqwestWrapperTrait {
    fn get_as_bytes(&self,path: String) -> Result<Bytes,Error>;
}

pub struct ReqwestWrapper;

impl ReqwestWrapperTrait for ReqwestWrapper {
    fn get_as_bytes(&self,path: String) -> Result<Bytes,Error>{
        let resp = reqwest::blocking::get(path)
            .map_err(|e|Error::from(e))?;
        resp.bytes()
            .map_err(|e|Error::from(e))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub url: Option<Url>,
    pub kind: ErrorKind
}

impl Error {
    fn from(err: reqwest::Error) -> Self {
        Self {
            url: err.url().map(|url|url.to_owned()),
            kind: 
                if err.is_builder() {
                    ErrorKind::Builder
                }
                else if err.is_body() {
                    ErrorKind::Body
                }
                else if err.is_status() {
                    ErrorKind::Status(err.status())
                }
                else if err.is_decode() {
                    ErrorKind::Decode
                }
                else if err.is_timeout() {
                    ErrorKind::Timeout
                }
                else if err.is_request() {
                    ErrorKind::Request
                }
                else if err.is_redirect() {
                    ErrorKind::Redirect
                }
                else if err.is_connect() {
                    ErrorKind::Connect
                }
                else {
                    ErrorKind::Unknown
                }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
	Builder,
	Request,
	Body,
	Decode,
	Redirect,
    Connect,
    Timeout,
	Status(Option<StatusCode>),
    Unknown
}
