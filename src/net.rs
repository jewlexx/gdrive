use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

pub fn get_unused_port() -> io::Result<u16> {
    // Binding to :0 requests that the operating system pick a free port for us.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    Ok(addr.port())
}

pub fn get_loopback() -> io::Result<SocketAddrV4> {
    let port = get_unused_port()?;
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);

    Ok(SocketAddrV4::new(ip_addr, port))
}
