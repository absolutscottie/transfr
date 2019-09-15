use std::io::Read;
use std::net::{TcpListener,TcpStream,SocketAddr};
use std::sync::mpsc::channel;
use std::thread;

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
    let mut buf_filename_len = [0 as u8; 8];
    match stream.read_exact(&mut buf_filename_len) {
        Ok(size) => read_filename(stream, to_i64_be(buf_filename_len)),
        Err(_) => return,
    }
}

fn read_filename(stream: TcpStream, length: i64) {

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
