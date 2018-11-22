use core::fmt;
use std::collections::HashMap;

use super::common::HTTPVersion;

static HORIZONTAL_LINE_RESPONSE: &str = "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< RESPONSE <<<<<<<<<<<<<<<<<<<<<<<<<<<<<";

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", *self as u8)
    }
}

pub type ReasonPhrase = String;

pub struct StatusLine {
    version: HTTPVersion,
    status_code: StatusCode,
    reason_phrase: ReasonPhrase,
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.version);
        write!(f, "{} ", self.status_code);
        write!(f, "{}", self.reason_phrase)
    }
}

impl fmt::Debug for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Response Header Field
/// * Location
/// * Server
/// * WWW-Authenticate
///
/// Entity Header Fields
/// * Allow
/// * Content-Encoding
/// * Content-Length
/// * Content-Type
/// * Expires
/// * Last-Modified
pub type Header = HashMap<String, String>;

pub struct Response {
    status_line: StatusLine,
    // Header
    header: Header,
    // Entity-Body
    body: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        let mut header = Header::new();
        header.insert("Content-Type".to_string(), "text/html".to_string());
        Response {
            status_line: StatusLine {
                version: HTTPVersion::new(1, 0),
                status_code: StatusCode::OK,
                reason_phrase: String::new(),
            },
            header,
            body: Some("Hello World!".to_string()),
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", &self.status_line);
        for (k, v) in &self.header {
            writeln!(f, "{}: {}", k, v);
        }
        writeln!(f, "");
        match &self.body {
            Some(body) => write!(f, "{}", body),
            &_ => Ok(())
        }
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", HORIZONTAL_LINE_RESPONSE);
        writeln!(f, "{}", &self.status_line);
        writeln!(f, "{:#?}", &self.header);
        writeln!(f, "{:?}", &self.body)
    }
}