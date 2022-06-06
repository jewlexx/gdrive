mod client;

use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT_INFO: client::ClientInfo = client::ClientInfo::new().unwrap();
}

fn main() {
    println!("Hello, world!");
}
