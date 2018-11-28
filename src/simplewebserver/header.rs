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
use std::str::FromStr;

use super::request::Method;

/// ## Allow
/// The Allow entity-header field lists the set of methods supported by
/// the resource identified by the Request-URI. The purpose of this field
/// is strictly to inform the recipient of valid methods associated with
/// the resource. The Allow header field is not permitted in a request
/// using the POST method, and thus should be ignored if it is received
/// as part of a POST entity.
struct Allow {
    methods: Vec<Method>,
}

impl FromStr for Allow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut methods = Vec::new();
        let arr = s.to_string().split(",");
        for item in arr {
            match Method::from_str(item) {
                Some(method) => methods.append(method),
                Err(e) => unimplemented!()
            }
        }
        Allow {
            methods
        }
    }
}

impl Display for Allow {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for item in self.methods {
            write!(f, "{},", item);
        }
        write!(f, "")
    }
}

enum Authorization {
    BaseAuthorization {
        username: String,
        password: String,
    }
}

impl FromStr for Authorization {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let cursor = s.splitn(2, " ");
        match cursor.next() {
            "Basic" => Authorization::BaseAuthorization {
                username: String::new(),
                password: String::new(),
            },
            _ => unimplemented!()
        }
    }
}

impl Display for Authorization {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

enum ContentEncoding {
    Gzip,
    Compress,
}

impl FromStr for ContentEncoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "x-gzip" => ContentEncoding::Gzip,
            "gzip" => ContentEncoding::Gzip,
            "x-compress" => ContentEncoding::Compress,
            "compress" => ContentEncoding::Compress
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
enum Text {
    /// Any document that contains text and is theoretically human readable
    Any,
}

impl FromStr for Text {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Text::Any,
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

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
            _ => Image::Any,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

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
            _ => Audio::Any,
        }
    }
}

impl Display for Audio {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

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
            _ => Video::Any,
        }
    }
}

impl Display for Video {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

enum Application {
    /// Any kind of binary data, especially data that will be executed or interpreted somehow.
    Any,
    OctetStream,
    JavaScript,
    OGG,
}

impl FromStr for Application {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Application::Any,
        }
    }
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
            "octet-stream" => Application::OctetStream,
            _ => Application::Any,
        }
    }
}

impl FromStr for Application {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            _ => Application::Any,
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

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
            _ => Multipart::Any,
        }
    }
}

impl Display for Multipart {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!()
    }
}

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
        match types.next() {
            Some(t) => match t {
                "text" => ContentType::Text(Text::from_str(types.next().unwrap()).unwrap()),
                "image" => ContentType::Image(Image::from_str(types.next().unwrap()).unwrap()),
                "audio" => ContentType::Audio(Audio::from_str(types.next().unwrap()).unwrap()),
                "video" => ContentType::Video(Video::from_str(types.next().unwrap()).unwrap()),
                "application" => ContentType::Application(Application::from_str(types.next().unwrap()).unwrap()),
                "multipart" => ContentType::Multipart(Multipart::from_str(types.next().unwrap()).unwrap())
            },
            _ => ContentType::None,
        }
    }
}


/// # Header Field Definitions
/// [[RFC1945, Section 10](https://tools.ietf.org/html/rfc1945#section-10)]
/// This section defines the syntax and semantics of all commonly used
/// HTTP/1.0 header fields. For general and entity header fields, both
/// sender and recipient refer to either the client or the server,
/// depending on who sends and who receives the message.
enum HeaderKey {
    Allow,
    Authorization,
    ContentEncoding,
    ContentLength,
    ContentType,
    Date,
    Expires,
    From,
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
}

enum HeaderValue {}