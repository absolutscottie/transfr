use std::io::Read;
use std::net::{TcpListener,TcpStream};
use std::thread;

use transfr;

#[derive(Debug)]
pub struct ServerInfo {
    pub addr: String,
    pub port: i32,
}

pub fn run(info: ServerInfo) {
    let listener = TcpListener::bind(format!("{}:{}", info.addr, info.port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => new_connection(stream),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn new_connection(stream: TcpStream) {
    thread::spawn(move || {handle_connection(stream)});
}

fn handle_connection(stream: TcpStream) {
    
}
