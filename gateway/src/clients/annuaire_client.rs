use common::{
    tokio::net::TcpStream,
    remoc::{self, rch},
    tokio,
    warp
};
use inter_services_messages::{
    Message, 
    MessageData, 
    ResponseData,
    annuaire::{AnnuaireMessage, AnnuaireSearchInput, User},
    Error
};
use std::env;

pub async fn annuaire_search(input: AnnuaireSearchInput) -> impl warp::Reply {
    match client(MessageData::Annuaire(AnnuaireMessage::Search(input))).await {
        Ok(res) => warp::reply::json(&res),
        Err(e) => warp::reply::json(&Error::from(e.as_ref()))
    }
}

pub async fn annuaire_register_post(input: User) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&vec![input]))
}

async fn client(msg: MessageData) -> Result<ResponseData, String> {
    let address = match env::var("AnnuaireClientAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4014")
    };
    // Establish TCP connection.
    let socket =
        TcpStream::connect(&address).await.unwrap();
    let (socket_rx, socket_tx) = socket.into_split();
    
    let (conn, tx, _rx): (_, _, rch::base::Receiver<()>) =
        remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
        .await.unwrap();
    tokio::spawn(conn);

    // Run client.
    connect(tx, msg).await
}


async fn connect(mut tx: rch::base::Sender<Message>, msg: MessageData) -> Result<ResponseData, String> {
    let (sender, receiver) = rch::oneshot::channel();
    tx.send(Message{data: msg, sender}).await.unwrap();

    match receiver.await {
        Ok(r) => r,
        Err(e) => Err(e.to_string())
    }
}