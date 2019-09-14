use std::io::Read;
use std::net::{TcpListener,TcpStream,SocketAddr};
use std::thread;

struct FileInfoRequest {
    name_len: i64,
    name: String,
    file_size: i64,
    checksum_len: i64,
    checksum: String,
}

struct FileInfoResponse {
    success: i8,

}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => handle_connection(&mut stream, addr),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn handle_connection(stream: &mut TcpStream, addr: SocketAddr) {
    println!("Connection from {:?}", addr);

    thread::spawn({ move || read(&mut stream) });
}

fn read(stream: &mut TcpStream) {
    //@todo error handling
    let mut buf_name_len: [u8; 8] = [0; 8];

    stream.read_exact(&mut buf_name_len);
    let name_len = to_i64be(buf_name_len);
}

fn to_i64be(bytes: [u8; 8]) -> i64 {
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
