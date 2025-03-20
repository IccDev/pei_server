pub mod database;
pub mod router;
pub mod models;
pub mod schema;
//mod clients;
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
use diesel::prelude::*;
//use dotenvy::dotenv;


pub fn establish_connection() -> PgConnection {
    //dotenv().ok();

    let database_url = match env::var("AnnuaireDatabaseAddress") {
        Ok(a) => a,
        Err(_) => String::from("postgres://icc_admin:icc_admin_2023@127.0.0.1:5434/postgres")
    };
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    let address = match env::var("AnnuaireAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4013")
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