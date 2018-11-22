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

#![feature(bufreader_buffer)]
extern crate core;

use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

mod common;
mod request;
mod response;

fn handle_client(stream: TcpStream) {
    let reader = &mut BufReader::new(&stream);
    handle_request(reader);
    let writer = BufWriter::new(&stream);
    send_response(writer);
}

fn handle_request(reader: &mut BufReader<&TcpStream>) {
    request::Request::from(reader);
}

fn send_response(mut writer: BufWriter<&TcpStream>) {
    // Write the header and the html body
    let res_data = response::Response::default();
    println!("{:?}", res_data);
    writer.write_fmt(format_args!("{}", res_data));
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