pub mod services;
mod server;

use server::server;
use crate::services::DatabaseService;

use std::env;
use common::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    tokio::{self, net::TcpListener},
    remoc::{self, rch},
    acteur::Acteur
};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();
    let acteur = Acteur::new();
    let _ = acteur.preload_service::<DatabaseService>();

    let address = match env::var("AnnuaireClientAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4014")
    };

    info!("Server: http://{}", address);

    // Listen for incoming TCP connection.
    let listener = TcpListener::bind(&address).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let acteur_clone = acteur.clone();
        tokio::task::spawn(async move {
            let (socket_rx, socket_tx) = socket.into_split();
            let (conn, _tx, rx): (_, rch::base::Sender<()>, _) =
                remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
                .await.unwrap();
            tokio::spawn(conn);
        
            // Run server.
            server(rx, acteur_clone.clone()).await;
        });
    }
}