pub(crate) mod clients;
pub(crate) mod router;
pub(crate) mod addresses;
pub(crate) mod result;
pub(crate) mod read_file;

use std::path::Path;
use common_crates::{
    tracing::{info, subscriber::set_global_default},
    tracing_subscriber::FmtSubscriber,
    tokio::{self, net::{TcpListener, TcpStream}},
    hyper::{
        client::conn::http1 as client_http1,
        server::conn::http1 as server_http1, 
        service::service_fn
    },
    hyper_util::rt::tokio::TokioIo,
    tokio_rustls::{
        self, 
        rustls::{
            pki_types::{CertificateDer, PrivateKeyDer},
            ServerConfig,
        }
    },
    tls_listener::TlsListener,
    rustls,
    dotenv::dotenv
};
use self::{
    router::router,
    addresses::*,
    result::GenericError,
    read_file::*
};
use common_crates::rustls_pemfile;
use std::{io, fs::File, sync::Arc};


#[tokio::main]
async fn main() -> Result<(), GenericError>  {
    dotenv().ok();
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    let gateway_add = gateway_address();
    let listener = TcpListener::bind(gateway_add).await?;


    info!("Listening on http://{}", gateway_add);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        let service = service_fn(move |mut req| {
            async move {
                let method = req.method().clone();
                let path = req.uri().path();
                let address = router(path, method).await;
                let uri_string = format!(
                    "http://{}{}",
                    address,
                    req.uri()
                        .path_and_query()
                        .map(|x| x.as_str())
                        .unwrap_or("/")
                );
                
                let uri = uri_string.parse().unwrap();
                *req.uri_mut() = uri;
    
                let host = req.uri().host().expect("uri has no host");
                let port = req.uri().port_u16().unwrap_or(80);
                let addr = format!("{}:{}", host, port);
                let client_stream = TcpStream::connect(addr).await.unwrap();
                let io = TokioIo::new(client_stream);

                let (mut sender, conn) = client_http1::handshake(io).await?;
                tokio::task::spawn(async move {
                    if let Err(err) = conn.await {
                        info!("Connection failed: {:?}", err);
                    }
                });

                sender.send_request(req).await
            }
        });

        tokio::task::spawn(async move {
            if let Err(err) = server_http1::Builder::new().serve_connection(io, service).await {
                info!("Failed to serve the connection: {:?}", err);
            }
        });
    }
}
