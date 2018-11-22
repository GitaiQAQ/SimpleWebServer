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