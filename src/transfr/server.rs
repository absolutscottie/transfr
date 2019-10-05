use std::io::Read;
use std::net::{TcpListener,TcpStream};
use std::thread;

use transfr::protocol;

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

fn new_connection(mut stream: TcpStream) {
    thread::spawn(move || {handle_connection(stream)});
}

fn handle_connection(mut stream: TcpStream) {
    //Handshake Phase
    let success = match protocol::server_handshake::handshake(&mut stream) {
        Ok(s) => s,
        Err(e) => {
            println!("Error in handshake: {}", e);
            false
        },
    };

    if(!success) {
        return;
    }

    //Setup Phase
    let success = match protocol::server_setup::setup_transfer(&mut stream) {
        Ok((name, size)) => (name, size),
        Err(e) => {
            println!("Error in transfer setup: {}", e);
            ("".to_string(), 0)
        },
    };

    if(success.0 == "" || success.1 <= 0) {
        return;
    }

    //Transfer Phase
}
