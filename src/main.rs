extern crate core;

use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::{TcpListener, TcpStream};
use std::io::Write;

static HORIZONTAL_LINE_REQUEST: &str = ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> REQUEST >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>";
static HORIZONTAL_LINE_RESPONSE: &str = "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< RESPONSE <<<<<<<<<<<<<<<<<<<<<<<<<<<<<";

fn handle_client(stream: TcpStream) {
    println!("{}", HORIZONTAL_LINE_REQUEST);

    let reader = BufReader::new(&stream);
    handle_request(reader);
    let mut writer = BufWriter::new(&stream);
    send_response(writer);
}

fn handle_request(reader: BufReader<&TcpStream>) {
    for line in reader.lines() {
        let req_data = line.unwrap();
        if req_data == "" {
            break;
        }
        println!("{}", req_data);
    }
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

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn main() {
    listener().unwrap()
}