extern crate core;

use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;
use std::io::Write;
use core::borrow::BorrowMut;
use std::io::BufReader;
use std::io::BufRead;

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines() {
        let request_data = line.unwrap();
        if request_data == "" {
            break;
        }
        println!("{}", request_data);
    }

    send_response(reader.into_inner());
}

fn send_response(mut stream: TcpStream) {
    // Write the header and the html body
    stream.write_fmt(format_args!("{}\n\r{}\n\r{}",
                                  "HTTP/1.0 200 OK",
                                  "Server: A Simple Web Server\r\nContent-Type: text/html\r\n",
                                  "Hello World!"));
}

fn listener() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
        listener.accept();
    }
    Ok(())
}

fn main() {
    listener().unwrap()
}