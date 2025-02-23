use std::env;
use std::net::SocketAddr;
use std::str::FromStr;


pub(crate) fn gateway_address<'a>() -> SocketAddr {
    match env::var("GatewayAddress") {
        Ok(a) => SocketAddr::from_str(&a).expect("No able to parse address!"),
        Err(_) => SocketAddr::from_str("127.0.0.1:4012").expect("No able to parse address!")
    }
}

pub(crate) fn mjib_address<'a>() -> String {
    match env::var("MjibAddress") {
        Ok(a) => a,
        Err(_) => "127.0.0.1:4014".to_string()
    }
}

pub(crate) fn gateway_key<'a>() -> String {
    match env::var("GatewayKey") {
        Ok(a) => a,
        Err(_) => "./certificates/key.pem".to_string()
    }
}

pub(crate) fn gateway_cert<'a>() -> String {
    match env::var("GatewayCert") {
        Ok(a) => a,
        Err(_) => "./certificates/cert.pem".to_string()
    }
}

pub(crate) fn annuaire_address<'a>() -> String {
    match env::var("AnnuaireAddress") {
        Ok(a) => a,
        Err(_) => "127.0.0.1:4013".to_string()
    }
}