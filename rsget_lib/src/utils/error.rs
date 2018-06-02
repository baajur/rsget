use std::string::FromUtf8Error;

use serde_json::Error as JsonError;
use std::{
    error::Error as StdError,
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    io::Error as IoError,
};

use hyper::Error as HyperError;

use http::uri::InvalidUri;
use http::header::ToStrError;

//use hyper::error::{Error as HyperError};
//use hyper::error::UriError;
use reqwest::{
    Error as ReqwestError,
    Response as ReqwestResponse,
    UrlError as ReqwestUrlError,
};

use http::Error as HttpError;

use hls_m3u8::Error as HlsError;


#[derive(Debug)]
pub struct RsgetError {
    details: String,
}

impl RsgetError {
    pub fn new(msg: &str) -> RsgetError {
        RsgetError{details: String::from(msg)}
    }
}

impl Display for RsgetError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,"{}",self.details)
    }
}

impl StdError for RsgetError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub enum StreamError {
    /// An error that occurred while formatting a string.
    Fmt(FmtError),
    /// An error from the `serde_json` crate while deserializing the body of an
    /// HTTP response.
    Json(JsonError),
    /// An error from the `hyper` crate while performing an HTTP request.
    Hyper(HyperError),
    /// An error from the `reqwest` crate while performing an HTTP request.
    Reqwest(ReqwestError),
    /// An error indicating a bad request when using `reqwest`.
    ReqwestBad(Box<ReqwestResponse>),
    /// An error indicating an invalid request when using `reqwest`.
    ReqwestInvalid(Box<ReqwestResponse>),
    /// An error indicating a parsing issue when using `reqwest`.
    ReqwestParse(ReqwestUrlError),
    /// RsgetError
    Rsget(RsgetError),
    /// IO-Error
    Io(IoError),
    /// UriError
    Uri(InvalidUri),
    /// ToStrError
    ToStr(ToStrError),
    /// HTTP Error
    Http(HttpError),
    /// HLS Error
    Hls(HlsError),
    /// FromUtf8error
    Utf8(FromUtf8Error),
}

impl Display for StreamError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for StreamError {
    fn description(&self) -> &str {
        match *self {
            StreamError::Fmt(ref inner) => inner.description(),
            StreamError::Hyper(ref inner) => inner.description(),
            StreamError::Json(ref inner) => inner.description(),
            StreamError::Reqwest(ref inner) => inner.description(),
            StreamError::ReqwestBad(_) => "Request bad",
            StreamError::ReqwestInvalid(_) => "Request invalid",
            StreamError::ReqwestParse(ref inner) => inner.description(),
            StreamError::Rsget(ref inner) => inner.description(),
            StreamError::Io(ref inner) => inner.description(),
            StreamError::Uri(ref inner) => inner.description(),
            StreamError::ToStr(ref inner) => inner.description(),
            StreamError::Http(ref inner) => inner.description(),
            StreamError::Hls(ref inner) => inner.description(),
            StreamError::Utf8(ref inner) => inner.description(),
        }
    }
}

impl From<FmtError> for StreamError {
    fn from(err: FmtError) -> Self {
        StreamError::Fmt(err)
    }
}


impl From<JsonError> for StreamError {
    fn from(err: JsonError) -> Self {
        StreamError::Json(err)
    }
}

impl From<HyperError> for StreamError {
    fn from(err: HyperError) -> Self {
        StreamError::Hyper(err)
    }
}

impl From<ReqwestError> for StreamError {
    fn from(err: ReqwestError) -> Self {
        StreamError::Reqwest(err)
    }
}

impl From<ReqwestUrlError> for StreamError {
    fn from(err: ReqwestUrlError) -> Self {
        StreamError::ReqwestParse(err)
    }
}

impl From<IoError> for StreamError {
    fn from(err: IoError) -> Self {
        StreamError::Io(err)
    }
}

impl From<InvalidUri> for StreamError {
    fn from(err: InvalidUri) -> Self {
        StreamError::Uri(err)
    }
}

impl From<ToStrError> for StreamError {
    fn from(err: ToStrError) -> Self {
        StreamError::ToStr(err)
    }
}

impl From<HttpError> for StreamError {
    fn from(err: HttpError) -> Self {
        StreamError::Http(err)
    }
}

impl From<HlsError> for StreamError {
    fn from(err: HlsError) -> Self {
        StreamError::Hls(err)
    }
}

impl From<FromUtf8Error> for StreamError {
    fn from(err: FromUtf8Error) -> Self {
        StreamError::Utf8(err)
    }
}
