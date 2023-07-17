use icc_common::{
    tokio::net::TcpStream,
    remoc::{self, rch},
    tokio
};
use inter_services_messages::{Message, MessageData, ResponseData};
use std::env;


pub async fn client(msg: MessageData) -> Result<ResponseData, String> {
    let address = match env::var("CovoiturageClient") {
        Ok(a) => a,
        Err(_) => String::from("192.168.1.5:4012")
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