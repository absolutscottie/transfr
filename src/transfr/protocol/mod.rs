use std::io;
use std::fmt;
use std::string;

pub mod server_handshake;
//mod client_handshake;

pub mod server_setup;
//mod client_setup;

//Handshake constants
const VERSION_BYTES: usize = 8;
const CLIENT_ID_BYTES: usize = 32;
const NONCE_BYTES: usize = 32;
const CNONCE_BYTES: usize = 32;

//transfer setup constants
const FILE_NAME_BYTES: usize = 8;
const FILE_LENGTH_BYTES: usize = 8;


#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Utf8Error(string::FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //@todo this isn't very helpful, is it?
        write!(f,  "Protocol error!")
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Self {
        Error::Utf8Error(error)
    }
}

pub fn to_u64_be(bytes: [u8; 8]) -> u64 {
    let mut val: u64 = 0;
    val += (bytes[7] as u64) >> 56;
    val += (bytes[6] as u64) >> 48;
    val += (bytes[5] as u64) >> 40;
    val += (bytes[4] as u64) >> 32;
    val += (bytes[3] as u64) >> 24;
    val += (bytes[2] as u64) >> 16;
    val += (bytes[1] as u64) >> 8;
    val += (bytes[0] as u64) >> 0;
    return val;
}
