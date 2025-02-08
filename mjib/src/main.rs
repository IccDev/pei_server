pub mod router;

use std::env;
use common_crates::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    tokio::{self, net::TcpListener},
    hyper::{
        service::service_fn,
        server::conn::http1,
    },
    hyper_util::rt::tokio::TokioIo
};
use router::router;



#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    let address = match env::var("MjibAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4014")
    };
    info!("Server: http://{}", address);

    // Listen for incoming TCP connection.
    let listener = TcpListener::bind(&address).await.unwrap();

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let io = TokioIo::new(stream);
        
                tokio::task::spawn(async move {
                    let service = service_fn(router);
        
                    if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                        info!("Failed to serve connection: {:?}", err);
                    }
                });
            },
            Err(e) => {
                info!("{e:#?}")
            }
        }
    }
}