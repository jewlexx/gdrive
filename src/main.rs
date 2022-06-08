mod client;
mod net;

use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use lazy_static::lazy_static;
use warp::Filter;

use client::credentials::ClientInfo;
use net::get_loopback;

const RESPONSE: &[u8] = include_bytes!("response.html");

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().unwrap();
}

fn handle_stream(mut stream: TcpStream, addr: SocketAddr) -> io::Result<()> {
    println!("Got connection");
    let mut buf = Vec::<u8>::new();

    // loop {
    let n = stream.read(&mut buf)?;

    // if n == 0 {
    //     return Ok(());
    // }

    let request = String::from_utf8_lossy(&buf[..n]);
    println!("Request: {}", request);

    stream.write_all(RESPONSE)?;
    // }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let listener = TcpListener::bind("127.0.0.1:0")?;

    let addr = get_loopback()?;
    let builder = hyper::Server::bind(&addr);

    // loop {
    match listener.accept() {
        Ok((socket, addr)) => handle_stream(socket, addr)?,
        Err(e) => println!("couldn't get client: {e:?}"),
    }
    // }

    Ok(())
}
