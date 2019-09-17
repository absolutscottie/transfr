use std::io::Read;
use std::net::{TcpListener,TcpStream,SocketAddr};
use std::sync::mpsc::channel;
use std::thread;

struct TransfrInfo {
    filename_length: i64,
    filename: String,
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut info = TransfrInfo {
        fliename_length: 0,
        filename: "",
    }

    read_filename(&mut )
}

fn read_filename(stream: TcpStream, info: mut TransfrInfo) {
    let mut buf = [u8, 8]; //64 bits
}

fn to_i64_be(bytes: [u8; 8]) -> i64 {
    let mut val: i64 = 0;
    val += (bytes[7] as i64) >> 56;
    val += (bytes[6] as i64) >> 48;
    val += (bytes[5] as i64) >> 40;
    val += (bytes[4] as i64) >> 32;
    val += (bytes[3] as i64) >> 24;
    val += (bytes[2] as i64) >> 16;
    val += (bytes[1] as i64) >> 8;
    val += (bytes[0] as i64) >> 0;

    return val;
}
