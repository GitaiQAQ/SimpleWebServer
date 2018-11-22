use core::fmt;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::net::{TcpStream};
use std::str::FromStr;
use super::common::HTTPVersion;

static HORIZONTAL_LINE_REQUEST: &str = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> REQUEST >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>";

#[derive(Debug)]
enum Method {
    GET,
    HEAD,
    POST,
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
            &_ => Err(())
        };
    }
}

#[derive(Debug)]
struct StatusLine {
    method: Method,
    uri: String,
    version: HTTPVersion,
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.method, self.uri, self.version)
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        return StatusLine {
            method: Method::GET,
            uri: String::new(),
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
            uri: String::from(result.next().unwrap()),
            version: HTTPVersion::from_str(result.next().unwrap()).unwrap(),
        });
    }
}

/// Request Header Fields
/// * Authorization
/// * From
/// * If-Modified-Since
/// * Referer
/// * User-Agent
type Header = HashMap<String, String>;

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

impl From<&mut BufReader<&TcpStream>> for Request {
    fn from(reader: &mut BufReader<&TcpStream>) -> Self {
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
                let mut tokens = s.splitn(2, ":");
                let key = tokens.next().unwrap().trim();
                let value = tokens.next().unwrap().trim();
                header.insert(key.to_string(), value.to_string());
            }
        }

        let mut body = None;
        {
            let value = header.get("Content-Length");
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

        print!("{:?}", req);

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