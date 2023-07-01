pub mod services;

use std::env;
use icc_common::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    tokio::{self, net::TcpListener},
    remoc::{self, rch},
    acteur::Acteur
};
use inter_services_messages::{Message, MessageData};
use services::{
    users,
    database::DatabaseService
};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();
    let acteur = Acteur::new();
    let _ = acteur.preload_service::<DatabaseService>();

    let address = match env::var("Address") {
        Ok(a) => a,
        Err(_) => String::from("192.168.1.3:4011")
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
            MessageData::RegisterUser(user) => {
                match users::create_user(user, acteur.clone()).await {
                    Ok(r) => {
                        req.sender.send(Ok(r)).unwrap();
                    },
                    Err(e) => {
                        req.sender.send(Err(e)).unwrap();
                    }
                } 
            },
            MessageData::ListUsers(user_token) => {
                match users::list_users(user_token, acteur.clone()).await {
                    Ok(r) => {
                        req.sender.send(Ok(r)).unwrap();
                    },
                    Err(e) => {
                        req.sender.send(Err(e)).unwrap();
                    }
                }
            },
            MessageData::LoginUser(login_form) => {
                match users::login_user(login_form, acteur.clone()).await {
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