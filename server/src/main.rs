use std::io::Read;
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io;
use std::fmt;
use std::string;

#[derive(Debug)]
enum TransfrProtocolError {
    IoError(io::Error),
    Utf8Error(string::FromUtf8Error),    
}

impl fmt::Display for TransfrProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Protocol error!")
    }
}

impl From<io::Error> for TransfrProtocolError {
    fn from(error: io::Error) -> Self {
        TransfrProtocolError::IoError(error)
    }
}

impl From<string::FromUtf8Error> for TransfrProtocolError {
    fn from(error: string::FromUtf8Error) -> Self {
        TransfrProtocolError::Utf8Error(error)
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
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
    let _filename = match read_file_name(stream) {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading filename: {}!", e);
            return //closes the connection per documentation
        }
    };
}

fn read_file_name(mut stream: TcpStream) -> Result<String, TransfrProtocolError> {

    let mut buf = [0 as u8; 8];
    stream.read_exact(&mut buf)?;

    let mut buf = vec![0 as u8; to_i64_be(buf)];
    stream.read_exact(&mut buf)?; 

    //pub fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>
    //@TODO i need to figure out why this compiles. To clarify: IMO it should compile without
    //the Ok(filename) part.
    return filename = String::from_utf8(buf)?;
}

fn to_i64_be(bytes: [u8; 8]) -> usize {
    let mut val: usize = 0;
    val += (bytes[7] as usize) >> 56;
    val += (bytes[6] as usize) >> 48;
    val += (bytes[5] as usize) >> 40;
    val += (bytes[4] as usize) >> 32;
    val += (bytes[3] as usize) >> 24;
    val += (bytes[2] as usize) >> 16;
    val += (bytes[1] as usize) >> 8;
    val += (bytes[0] as usize) >> 0;

    return val;
}
