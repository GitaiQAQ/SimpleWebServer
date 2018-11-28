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

use core::borrow::Borrow;
use std::io::{Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub mod common;
pub mod status;
pub mod header;
pub mod request;
pub mod response;

fn handle_client(stream: &mut TcpStream) {
    let req = request::Request::from(stream.borrow());
    debug!("{:?}", req);

    // Write the header and the html body
    let res = response::Response::not_found();
    debug!("{:?}", res);
    stream.write_fmt(format_args!("{}", res));
}

pub struct App {
    addr: SocketAddr,
}

impl App {
    pub fn new(addr: SocketAddr) -> Self {
        App {
            addr
        }
    }

    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;
        info!("Listening on {}", self.addr);
        listener.set_ttl(10)?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            handle_client(&mut stream?)
        }
        Ok(())
    }
}