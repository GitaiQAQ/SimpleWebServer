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

//! HTTP header
use core::fmt::{Display, Error, Formatter};
use std::collections::HashMap;
use std::convert::From;
use std::str::FromStr;

use chrono::{DateTime, Local};

use super::request::Method;

/// ## Allow
/// The Allow entity-header field lists the set of methods supported by
/// the resource identified by the Request-URI. The purpose of this field
/// is strictly to inform the recipient of valid methods associated with
/// the resource. The Allow header field is not permitted in a request
/// using the POST method, and thus should be ignored if it is received
/// as part of a POST entity.
#[derive(Debug)]
struct Allow {
    methods: Vec<Method>,
}

impl FromStr for Allow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut methods = Vec::new();
        let arr = s.split(",");
        for item in arr {
            match Method::from_str(item) {
                Ok(method) => methods.push(method),
                Err(e) => unimplemented!()
            }
        }
        Ok(Allow {
            methods
        })
    }
}

impl Display for Allow {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for item in &self.methods {
            write!(f, "{},", item);
        }
        write!(f, "")
    }
}

#[derive(Debug)]
enum Authorization {
    BaseAuthorization {
        username: String,
        password: String,
    }
}

impl FromStr for Authorization {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut cursor = s.splitn(2, " ");
        match cursor.next() {
            Some("Basic") => Ok(Authorization::BaseAuthorization {
                username: String::new(),
                password: String::new(),
            }),
            _ => unimplemented!()
        }
    }
}

impl Display for Authorization {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum ContentEncoding {
    Gzip,
    Compress,
}

impl FromStr for ContentEncoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "x-gzip" => Ok(ContentEncoding::Gzip),
            "gzip" => Ok(ContentEncoding::Gzip),
            "x-compress" => Ok(ContentEncoding::Compress),
            "compress" => Ok(ContentEncoding::Compress),
            _ => unimplemented!()
        }
    }
}

impl Display for ContentEncoding {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

type ContentLength = usize;

/// # Multipurpose Internet Mail Extensions (MIME) type
#[derive(Debug)]
enum Text {
    /// Any document that contains text and is theoretically human readable
    Any,
}

impl FromStr for Text {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Ok(Text::Any),
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum Image {
    /// Any kind of image. Videos are not included, though animated images (like animated GIF) are
    /// described with an image type.
    Any,
    GIF,
    JPEG,
    PNG,
    SVG,
    WEBP,
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Ok(Image::Any),
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum Audio {
    /// Any kind of audio file
    Any,
    WAVE,
    WAV,
    WEBM,
    OGG,
}

impl FromStr for Audio {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Ok(Audio::Any),
        }
    }
}

impl Display for Audio {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum Video {
    /// Any kind of video file
    Any,
    WEBM,
    OGG,
}

impl FromStr for Video {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Ok(Video::Any),
        }
    }
}

impl Display for Video {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum Application {
    /// Any kind of binary data, especially data that will be executed or interpreted somehow.
    Any,
    OctetStream,
    JavaScript,
    OGG,
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

impl FromStr for Application {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "octet-stream" => Ok(Application::OctetStream),
            _ => Ok(Application::Any),
        }
    }
}

#[derive(Debug)]
enum Multipart {
    /// Multipart types indicate a category of document broken into pieces, often with different
    /// MIME types. They represent a composite document.
    Any,
    FormData,
    ByteRanges,
}

impl FromStr for Multipart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Ok(Multipart::Any),
        }
    }
}

impl Display for Multipart {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum ContentType {
    None,
    Text(Text),
    Image(Image),
    Audio(Audio),
    Video(Video),
    Application(Application),
    Multipart(Multipart),
}

impl FromStr for ContentType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut types = s.splitn(2, "/");
        Ok(match types.next() {
            Some(t) => match t {
                "text" => ContentType::Text(Text::from_str(types.next().unwrap()).unwrap()),
                "image" => ContentType::Image(Image::from_str(types.next().unwrap()).unwrap()),
                "audio" => ContentType::Audio(Audio::from_str(types.next().unwrap()).unwrap()),
                "video" => ContentType::Video(Video::from_str(types.next().unwrap()).unwrap()),
                "application" => ContentType::Application(Application::from_str(types.next().unwrap()).unwrap()),
                "multipart" => ContentType::Multipart(Multipart::from_str(types.next().unwrap()).unwrap()),
                _ => unimplemented!()
            },
            _ => ContentType::None,
        })
    }
}

type Date = DateTime<Local>;
type Expires = Date;
// type From = String;
type IfModifiedSince = Date;
type LastModified = Date;
type Location = String;
type Referer = String;
type Server = String;
type UserAgent = String;
type WWWAuthenticate = String;
type Accept = ContentType;

#[derive(Debug)]
enum Charset {}

type AcceptCharset = Charset;
type AcceptEncoding = ContentType;
type AcceptLanguage = String;
type ContentLanguage = String;
type Link = String;
type MIMEVersion = String;
type RetryAfter = String;
type Title = String;
type URI = String;


/// # Header Field Definitions
/// [[RFC1945, Section 10](https://tools.ietf.org/html/rfc1945#section-10)]
/// This section defines the syntax and semantics of all commonly used
/// HTTP/1.0 header fields. For general and entity header fields, both
/// sender and recipient refer to either the client or the server,
/// depending on who sends and who receives the message.
#[derive(Debug, Eq, PartialEq)]
pub enum HeaderName {
    Allow,
    Authorization,
    ContentEncoding,
    ContentLength,
    ContentType,
    Date,
    Expires,
    // From,
    IfModifiedSince,
    LastModified,
    Location,
    // Pragma,
    Referer,
    Server,
    UserAgent,
    WWWAuthenticate,
    /// ## Additional Header Field Definitions
    /// [[RFC1945, Appendix D.2](https://tools.ietf.org/html/rfc1945#appendix-D.2)]
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    ContentLanguage,
    Link,
    MIMEVersion,
    RetryAfter,
    Title,
    URI,
    Other(&'static str)
}

impl From<&'static str> for HeaderName {
    fn from(item: &'static str) -> Self {
        match item {
            "Allow" | "allow" => HeaderName::Allow,
            "Authorization" | "authorization" => HeaderName::Authorization,
            "Content-Encoding" | "content-encoding" => HeaderName::ContentEncoding,
            "Content-Length" | "content-length" => HeaderName::ContentLength,
            "Content-Type" | "content-type" => HeaderName::ContentType,
            "Date" | "date" => HeaderName::Date,
            "Expires" | "expires" => HeaderName::Expires,
            // "from" => HeaderName::From,
            "If-Modified-Since" | "if-modified-since" => HeaderName::IfModifiedSince,
            "Last-Modified" | "last-modified" => HeaderName::LastModified,
            "Location" | "location" => HeaderName::Location,
            "Referer" | "referer" => HeaderName::Referer,
            "Server" | "server" => HeaderName::Server,
            "User-Agent" | "user-agent" => HeaderName::UserAgent,
            "WWW-Authenticate" | "www-authenticate" => HeaderName::WWWAuthenticate,
            "Accept" | "accept" => HeaderName::Accept,
            "Accept-Charset" | "accept-charset" => HeaderName::AcceptCharset,
            "Accept-Encoding" | "accept-encoding" => HeaderName::AcceptEncoding,
            "Accept-Language" | "accept-language" => HeaderName::AcceptLanguage,
            "Content-Language" | "content-language" => HeaderName::ContentLanguage,
            "Link" | "link" => HeaderName::Link,
            "MIME-Version" | "mime-version" => HeaderName::MIMEVersion,
            "Retry-After" | "retry-after" => HeaderName::RetryAfter,
            "Title" | "title" => HeaderName::Title,
            "Uri" | "uri" => HeaderName::URI,
            e => HeaderName::Other(e),
        }
    }
}

pub type Header = HashMap<String, String>;