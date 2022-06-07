use std::{io, net::TcpListener};

pub fn get_unused_port() -> io::Result<u16> {
    // Binding to :0 requests that the operating system pick a free port for us.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    Ok(addr.port())
}
