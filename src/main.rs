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

use client::{credentials::ClientInfo, AUTH_ENDPOINT};
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

#[derive(Debug)]
struct Query {
    pub code: Option<String>,
}

impl Query {
    pub fn from_query_string(query: &str) -> Result<Self, String> {
        let opts = query.split('&');

        let mut query = Self { code: None };

        for opt in opts {
            let mut kv = opt.split('=');

            match kv.next() {
                Some("code") => query.code = kv.last().map(String::from),
                Some("error") => {
                    if let Some(err) = kv.last() {
                        return Err(String::from(err));
                    }
                }
                _ => continue,
            }
        }

        Ok(query)
    }
}

fn get_redirect() -> String {
    let info = &CLIENT_INFO.credentials;
    format!("{AUTH_ENDPOINT}/?client_id={}&redirect_uri=http://127.0.0.1&response_type=code&access_type=offline", info.client_id)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize client info straight away
    lazy_static::initialize(&CLIENT_INFO);

    let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(8);

    // let listener = TcpListener::bind("127.0.0.1:0")?;

    {
        let tx = tx.clone();
        ctrlc::set_handler(move || {
            tx.blocking_send(1).unwrap();
        })?;
    }

    let svc = make_service_fn(|socket: &AddrStream| async move {
        Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async move {
            if req.uri() == "/" {
                let res = Response::builder()
                    .status(302)
                    .header("Location", get_redirect())
                    .body(Body::from(""));
                return res;
            }
            let query_string = req.uri().query().unwrap_or("");
            let query = Query::from_query_string(query_string);
            println!("Got request");
            Response::builder()
                .status(200)
                .body(Body::from(format!("Hello, {:?}!", query)))
        }))
    });

    let addr = get_loopback()?;
    println!("Listening on http://{}", addr);
    let server = hyper::Server::bind(&addr).serve(svc);

    let with_grace = server.with_graceful_shutdown(async move {
        rx.recv().await.expect("Failed to recieve");
    });

    with_grace.await?;

    Ok(())
}
