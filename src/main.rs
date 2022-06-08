mod client;
mod net;

use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use lazy_static::lazy_static;

use client::credentials::ClientInfo;

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().unwrap();
}

fn handle_stream(mut stream: TcpStream, addr: SocketAddr) -> io::Result<()> {
    let mut buf = Vec::<u8>::new();

    loop {
        let n = stream.read_to_end(&mut buf)?;

        if n == 0 {
            return Ok(());
        }

        let request = String::from_utf8_lossy(&buf[..n]);
        println!("Request: {}", request);

        let response = format!("{}", request);
        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let listener = TcpListener::bind("127.0.0.1:0")?;

    println!("Listening on http://{}", listener.local_addr()?);

    loop {
        match listener.accept() {
            Ok((socket, addr)) => handle_stream(socket, addr)?,
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }

    Ok(())
}
