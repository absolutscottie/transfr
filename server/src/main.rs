use std::io::Read;
use std::net::{TcpListener,TcpStream,SocketAddr};
use std::sync::mpsc::channel;
use std::thread;

struct TransfrInfo {
    file_name: String,
    file_length: i64,
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
        file_name: "",
        file_length: 0,
    }

    read_filename(&mut )
}

fn read_filename(stream: &mut TcpStream, info: &mut TransfrInfo) -> Result<()> {
    //buf1 represents a 64bit unsigned integer which represents the
    //length of the name of the file being transferred.
    let mut buf1 = [u8, 8]; //64 bits
    stream.read_exact(&mut buf1)?;

    //buf2 is created with the previously read size and is filled
    //with bytes representing the file name
    let mut buf2 = [u8, to_i64_be(buf1)]
    stream.read_exact[&mut buf2]?;

    info.file_name = str::from_ut8(&buf2).unwrap();

    Ok(())
}

fn read_file_length(stream: &mut TcpStream) -> Result<()> {

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
