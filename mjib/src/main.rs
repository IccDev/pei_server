pub mod router;
mod addresses;
pub mod database;
pub mod routes;

use std::env;
use common_crates::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    tokio::{self, net::TcpListener},
    hyper::{
        service::service_fn,
        server::conn::http1,
        Request, 
        body::Incoming as IncomingBody
    },
    hyper_util::rt::tokio::TokioIo,
    dotenv::dotenv
};
use router::router;
use self::addresses::mjib_address;
use crate::database::DBService;
use std::sync::LazyLock;



static DB: LazyLock<DBService> = LazyLock::new(DBService::init);

#[tokio::main]
async fn main() {
    dotenv().ok();
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();
    DB.connect().await;

    let address = mjib_address();
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