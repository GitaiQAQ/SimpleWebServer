// Copyright 2018 Gitai<i@gitai.me> All rights reserved.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall
// be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR
// ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
// CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! HTTP Response
use core::fmt;
use std::collections::HashMap;

use super::common::HTTPVersion;
use super::status::StatusCode;

static HORIZONTAL_LINE_RESPONSE: &str = "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< RESPONSE <<<<<<<<<<<<<<<<<<<<<<<<<<<<<";

pub type ReasonPhrase = String;

pub struct StatusLine {
    version: HTTPVersion,
    status_code: StatusCode,
    reason_phrase: ReasonPhrase,
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.version);
        write!(f, "{} ", self.status_code.to_u16());
        write!(f, "{}", self.status_code)
    }
}

impl fmt::Debug for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// # Header Field Definitions
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

impl Response {
    pub fn not_found() -> Self {
        Response {
            status_line: StatusLine {
                version: HTTPVersion::new(1, 0),
                status_code: StatusCode::NotFound,
                reason_phrase: String::new(),
            },
            header: Header::new(),
            body: None,
        }
    }
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