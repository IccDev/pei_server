mod router;
mod clients;

use std::env;
use icc_common::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    hyper::{Body, Request, Response, server::{Server, conn::AddrIncoming}},
    hyper::service::{make_service_fn, service_fn},
    tokio,
    hyper_rustls::{TlsAcceptor},
    rustls,
    rustls_pemfile
};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use self::router::router;


//use hyper::{Body, Method, Request, Response, Server, StatusCode};

use std::io;
use std::fs::File;

// point d'entrer
async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match router(req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(Response::new(Body::from("Hello World")))
    }
}


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    let address = match env::var("GatewayAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4012")
    };

    // Load public certificate.
    //let certfile = File::open("/usr/local/bin/icc_ban_cert.pem").unwrap();
    //let certfile = File::open("/usr/local/bin/0000_csr-certbot.pem").unwrap();
    //let certfile = File::open("./0000_csr-certbot.pem").unwrap();
    //let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    //let certs = rustls_pemfile::certs(&mut reader).unwrap();
    //let certs = certs.into_iter().map(rustls::Certificate).collect();
/*
    // Load private key. (see `examples/server.rs`)
    //let keyfile = File::open("/usr/local/bin/icc_ban_key.pem").unwrap();
    let keyfile = File::open("/usr/local/bin/0000_key-certbot.pem").unwrap();
    //let keyfile = File::open("./0000_key-certbot.pem").unwrap();
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader).unwrap();
    let key = rustls::PrivateKey(keys[0].clone());
    
    
    
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from_str(address.as_ref()).expect("No able to parse address!");
    let incoming = AddrIncoming::bind(&addr).unwrap();
    let acceptor = TlsAcceptor::builder()
        .with_single_cert(certs, key).unwrap()
        .with_all_versions_alpn()
        .with_incoming(incoming);

    info!("Server: https://{}", address);
    let make_service = make_service_fn(|_conn| async move {
        Ok::<_, Infallible>(service_fn(handle))
    });
    
    let server = Server::builder(acceptor).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    */


    info!("Server: http://{}", address);
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from_str(address.as_ref()).expect("No able to parse address!");
    let make_service = make_service_fn(|_conn| async move {
        Ok::<_, Infallible>(service_fn(handle))
    });
    
    let server = Server::bind(&addr).serve(make_service);
    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
