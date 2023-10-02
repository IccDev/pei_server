pub mod services;

use std::env;
use icc_common::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    tokio::{self, net::TcpListener},
    remoc::{self, rch},
    acteur::Acteur
};
use inter_services_messages::{Message, MessageData, UserMessageData};
use services::{
    users,
    KeycloakAPIService,
    DatabaseService
};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();
    let acteur = Acteur::new();
    let _ = acteur.preload_service::<KeycloakAPIService>();
    let _ = acteur.preload_service::<DatabaseService>();

    let address = match env::var("UserClientAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4013")
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


// This would be run on the server.
async fn server(mut rx: rch::base::Receiver<Message>, acteur: Acteur) {
    while let Some(req) = rx.recv().await.unwrap() {
        match req.data {
            MessageData::User(UserMessageData::LoginUser(login_form)) => {
                match users::login(login_form, acteur.clone()).await {
                    Ok(r) => {
                        req.sender.send(Ok(r)).unwrap();
                    },
                    Err(e) => {
                        req.sender.send(Err(e)).unwrap();
                    }
                }
            }
            MessageData::User(UserMessageData::Annuaire(search)) => {
                match users::search_stars(search, acteur.clone()).await {
                    Ok(r) => {
                        req.sender.send(Ok(r)).unwrap();
                    },
                    Err(e) => {
                        req.sender.send(Err(e)).unwrap();
                    }
                }
            }
        }
    }
}