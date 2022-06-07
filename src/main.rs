mod client;
mod net;

use lazy_static::lazy_static;

use client::credentials::ClientInfo;

lazy_static! {
    static ref CLIENT_INFO: ClientInfo = ClientInfo::new().unwrap();
}

fn main() {
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);
    println!("Hello, world!");
}
