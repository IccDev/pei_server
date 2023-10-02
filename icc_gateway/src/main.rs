mod router;
mod clients;

use std::env;
use icc_common::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    hyper::{Body, Request, Response, server::Server},
    hyper::service::{make_service_fn, service_fn},
    tokio
};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use self::router::router;

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
