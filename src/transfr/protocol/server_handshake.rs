use std::net::TcpStream;
use transfr::protocol::Error;

pub fn handshake(mut stream: &TcpStream) -> Result<bool, Error> {
    let version = read_version(&mut stream)?;
    let client_id = read_client_id(&mut stream)?;

    Ok(true)
}

fn read_version(mut stream: &TcpStream) -> Result<i64, Error> {
    Ok(0)
}

fn read_client_id(mut stream: &TcpStream) -> Result<String, Error> {
    Ok("".to_string())
}
