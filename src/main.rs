mod client;
mod net;

use std::net::TcpListener;

use lazy_static::lazy_static;

use client::credentials::ClientInfo;

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().unwrap();
}

fn main() -> anyhow::Result<()> {
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let listener = TcpListener::bind("127.0.0.1:0")?;

    println!("Listening on {}", listener.local_addr()?);

    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }

    Ok(())
}
