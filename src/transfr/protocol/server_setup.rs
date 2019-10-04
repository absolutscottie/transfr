use std::io::Read;
use std::net::TcpStream;
use transfr::protocol;
use transfr::protocol::{Error, VERSION_BYTES, FILE_LENGTH_BYTES};

pub fn setup_transfer(mut stream: &TcpStream) -> Result<(String, u64), Error> {
    let filename = read_file_name(&stream)?;
    let file_length = read_file_length(&stream)?;

    Ok((filename, file_length))
}

fn read_file_name(mut stream: &TcpStream) -> Result<String, Error> {
    let mut buf = [0 as u8; VERSION_BYTES];
    stream.read_exact(&mut buf)?;

    let mut buf = vec![0 as u8; protocol::to_u64_be(buf) as usize];
    stream.read_exact(&mut buf)?;

    let filename = String::from_utf8(buf)?;
    Ok(filename)
}

fn read_file_length(mut stream: &TcpStream) -> Result<u64, Error> {
    let mut buf = [0 as u8; FILE_LENGTH_BYTES];
    stream.read_exact(&mut buf)?;
    Ok(protocol::to_u64_be(buf))
}
