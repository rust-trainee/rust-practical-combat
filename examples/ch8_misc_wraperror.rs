use std::fmt;
use std::fs::File;
use std::io;
use std::net;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(value: io::Error) -> Self {
        UpstreamError::IO(value)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(value: net::AddrParseError) -> Self {
        UpstreamError::Parsing(value)
    }
}
fn main() -> Result<(), UpstreamError> {
    let _f = File::open("ivisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;
    Ok(())
}
