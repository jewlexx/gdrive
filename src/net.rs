use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
};

pub fn get_unused_port() -> io::Result<u16> {
    // Binding to :0 requests that the operating system pick a free port for us.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    Ok(addr.port())
}

pub fn get_loopback() -> io::Result<SocketAddr> {
    let port = get_unused_port()?;
    let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    Ok(SocketAddr::new(ip_addr, port))
}
