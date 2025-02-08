mod router;
mod clients;

use std::env;
use common::{
    tracing::subscriber::set_global_default,
    tracing_subscriber::FmtSubscriber,
    tokio,
    warp
};
use std::net::SocketAddr;
use std::str::FromStr;
use self::router::routes;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    let address = match env::var("GatewayAddress") {
        Ok(a) => a,
        Err(_) => String::from("127.0.0.1:4012")
    };

    let addr = SocketAddr::from_str(address.as_ref()).expect("No able to parse address!");
    warp::serve(routes()).run(addr).await
}
