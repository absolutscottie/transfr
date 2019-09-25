use std::net::TcpStream;

use transfr::protocol;
use transfr::protocol::Error;

pub fn handshake(mut stream: &TcpStream) -> Result<bool, Error> {
    let version = match read_version(&mut stream) {
        Ok(version) => version,
        Err(e) => println!("{}", e),
    };

    let client_id = match read_client_id(&mut stream) {
        Ok(id) => id,
        Err(e) => println!("{}", e),
    };

    Ok(true)
}

fn read_version(mut stream: &TcpStream) -> Result<i64, Error> {
    Ok(0)
}

fn read_client_id(mut stream: &TcpStream) -> Result<String, Error> {
    Ok("".to_string())
}
