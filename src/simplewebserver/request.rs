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

use core::fmt;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::net::TcpStream;
use std::str::FromStr;

use super::common::HTTPVersion;
use super::header::{Header, HeaderName};

static HORIZONTAL_LINE_REQUEST: &str = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> REQUEST >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>";

/// # Request Methods
///
#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
    /// ## Additional Request Methods
    /// [[RFC1945, Appendix D.1](https://tools.ietf.org/html/rfc1945#appendix-D.1)]
    PUT,
    DELETE,
    LINK,
    UNLINK,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        return match s {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),

            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "LINK" => Ok(Method::LINK),
            "UNLINK" => Ok(Method::UNLINK),
            _ => Err(())
        };
    }
}

/// The URI is structured as follows:
///
/// ```notrust
/// abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
/// |-|   |-------------------------------||--------| |-------------------| |-----|
///  |                  |                       |               |              |
/// scheme          authority                 path            query         fragment
/// ```
#[derive(Debug)]
struct URI {
    original_url: String,
    authority_start: usize,
    socket_start: usize,
    path_start: usize,
    query_start: usize,
    fragment_start: usize,
}


impl URI {
    fn new() -> Self {
        URI {
            original_url: String::new(),
            authority_start: 0,
            socket_start: 0,
            path_start: 0,
            query_start: 0,
            fragment_start: 0,
        }
    }

    fn scheme(&self) -> &str {
        if self.authority_start != 0 {
            return &self.original_url[..self.authority_start - 3];
        } else {
            return &self.original_url[..self.socket_start - 3];
        }
    }

    fn protocol(&self) -> &str {
        self.scheme()
    }

    fn secure(&self) -> &str {
        &self.original_url[self.authority_start..self.socket_start]
    }

    fn socket(&self) -> &str {
        &self.original_url[self.socket_start..self.path_start]
            .trim_left_matches("@")
    }

    fn path(&self) -> &str {
        &self.original_url[self.path_start..self.query_start]
    }

    fn query(&self) -> &str {
        &self.original_url[self.query_start..self.fragment_start]
    }

    fn fragment(&self) -> &str {
        &self.original_url[self.fragment_start..]
    }
}

impl FromStr for URI {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let scheme_end = match s.find("://") {
            Some(i) => i + 3,
            _ => 0
        };

        let mut url = URI {
            original_url: s.to_string(),
            authority_start: 0,
            socket_start: 0,
            path_start: 0,
            query_start: 0,
            fragment_start: 0,
        };

        let s = &s[scheme_end..];

        let mut i = scheme_end;
        for c in s.chars() {
            if c == '@' {
                if url.socket_start == 0 {
                    url.authority_start = scheme_end;
                }
                if url.socket_start == 0 {
                    url.socket_start = i;
                }
            } else if c == '/' {
                if url.path_start == 0 {
                    url.path_start = i;
                }
                if url.socket_start == 0 {
                    url.socket_start = scheme_end;
                }
            } else if c == '?' {
                url.query_start = i;
            } else if c == '#' {
                url.fragment_start = i;
            }
            i += 1;
        }
        Ok(url)
    }
}


//let url = "abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1".parse::<URI>().unwrap();
//println!("{:?}", url.scheme());
//println!("{:?}", url.secure());
//println!("{:?}", url.socket());
//println!("{:?}", url.path());
//println!("{:?}", url.query());
//println!("{:?}", url.fragment());


#[derive(Debug)]
struct StatusLine {
    method: Method,
    uri: URI,
    version: HTTPVersion,
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.method, self.uri.original_url, self.version)
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        return StatusLine {
            method: Method::GET,
            uri: URI::new(),
            version: HTTPVersion::default(),
        };
    }
}

impl FromStr for StatusLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut result = s.split_whitespace();
        return Ok(StatusLine {
            method: Method::from_str(result.next().unwrap()).unwrap(),
            uri: result.next().unwrap().parse::<URI>().unwrap(),
            version: HTTPVersion::from_str(result.next().unwrap()).unwrap(),
        });
    }
}

pub struct Request {
    status_line: StatusLine,
    // Header
    header: Header,
    // Entity-Body
    body: Option<String>,
}

impl fmt::Display for Request {
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

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", HORIZONTAL_LINE_REQUEST);
        writeln!(f, "{}", &self.status_line);
        writeln!(f, "{:#?}", &self.header);
        writeln!(f, "{:?}", &self.body)
    }
}

impl From<&TcpStream> for Request {
    fn from(stream: &TcpStream) -> Self {
        let ref mut reader = BufReader::new(stream);
        let status_line;
        let mut header = Header::new();
        {
            let mut cursor = reader.lines();
            status_line = StatusLine::from_str(&cursor.next().unwrap().unwrap()).unwrap();

            for line in cursor {
                let s = &line.unwrap().trim().to_string();
                if s.is_empty() {
                    break;
                }
                let mut cursor = s.splitn(2, ":");
                let key = cursor.next().unwrap();
                let value = cursor.next().unwrap();
                header.insert(key.to_string(), value.to_string());
            }
        }

        let mut body = None;
        {
            let value = header.get("content-length");
            if value.is_some() {
                let content_length = usize::from_str(value.unwrap());
                if content_length.is_ok() {
                    let mut buffer = vec!('\0' as u8; content_length.unwrap());
                    reader.read_exact(&mut buffer[..]);
                    body = Some(String::from_utf8(buffer).unwrap())
                }
            }
        }

        let req = Request {
            status_line,
            header,
            body,
        };

        req

//        for line in reader.lines() {
//            let req_data = line.unwrap();
//            if req_data == "" {
//                break;
//            }
//            println!("{}", req_data);
//        }
    }
}