use std::{io, net::TcpListener};

pub fn get_unused_port() -> io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    Ok(addr.port())
}
