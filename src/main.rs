#![feature(bufreader_buffer)]
extern crate core;

use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::collections::HashMap;
use core::fmt;
use std::io::Read;

static HORIZONTAL_LINE_REQUEST: &str = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> REQUEST >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>";
static HORIZONTAL_LINE_RESPONSE: &str = "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< RESPONSE <<<<<<<<<<<<<<<<<<<<<<<<<<<<<";

#[derive(Debug)]
enum Method {
    GET, HEAD, POST,
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
        }
    }
}

#[derive(Debug)]
struct HTTPVersion {
    major: u8,
    minor: u8,
}

impl fmt::Display for HTTPVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP/{}.{}", self.major, self.minor)
    }
}

impl FromStr for HTTPVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut result = s.split(|c|
            c == '/' ||
                c == '.').filter(|k| !k.is_empty());
        result.next(); // HTTP
        Ok(HTTPVersion {
            major: u8::from_str(result.next().unwrap()).unwrap(),
            minor: u8::from_str(result.next().unwrap()).unwrap(),
        })
    }
}

type Header = HashMap<String, String>;

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
            version: HTTPVersion {
                major: 0,
                minor: 0,
            },
        }
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
        })
    }
}

struct Request {
    status_line: StatusLine,
    // Header
    header: Header,
    // Entity-Body
    body: Option<String>,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", HORIZONTAL_LINE_REQUEST);
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
            status_line= StatusLine::from_str(&cursor.next().unwrap().unwrap()).unwrap();

            for line in cursor {
                let s = &line.unwrap().trim().to_string();
                if s.is_empty() {
                    break
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

        let req = Request{
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

fn handle_client(stream: TcpStream) {
    let reader = &mut BufReader::new(&stream);
    handle_request(reader);
    let writer = BufWriter::new(&stream);
    send_response(writer);
}

fn handle_request(reader: &mut BufReader<&TcpStream>) {
    Request::from(reader);

//    for line in reader.lines() {
//        let req_data = line.unwrap();
//        if req_data == "" {
//            break;
//        }
//        println!("{}", req_data);
//    }
}

fn send_response(mut writer: BufWriter<&TcpStream>) {
    println!("{}", HORIZONTAL_LINE_RESPONSE);

    // Write the header and the html body
    let res_data = format_args!("{}\r\n{}\r\n{}",
                                "HTTP/1.0 200 OK",
                                "Server: A Simple Web Server\r\nContent-Type: text/html\r\n",
                                "Hello World!").to_string();

    println!("{}", res_data);
    writer.write(res_data.as_bytes());
}

fn listener() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    listener.set_ttl(10)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn main() {
    listener().unwrap()
}