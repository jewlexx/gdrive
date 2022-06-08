mod client;
mod net;

use std::{
    convert::Infallible,
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::mpsc::channel,
};

use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response,
};
use lazy_static::lazy_static;

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

    let (tx, rx) = tokio::sync::oneshot::channel::<u8>();

    let listener = TcpListener::bind("127.0.0.1:0")?;

    let svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async move {
                Ok::<_, Infallible>(Response::new(Body::from(format!(
                    "Hello, {}!",
                    remote_addr
                ))))
            }))
        }
    });

    let addr = get_loopback()?;
    let server = hyper::Server::bind(&addr).serve(svc);

    server.with_graceful_shutdown(async move {});

    // loop {
    match listener.accept() {
        Ok((socket, addr)) => handle_stream(socket, addr)?,
        Err(e) => println!("couldn't get client: {e:?}"),
    }
    // }

    Ok(())
}
