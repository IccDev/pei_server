use std::env;
use std::net::SocketAddr;
use std::str::FromStr;



pub(crate) fn mjib_address() -> String {
    match env::var("MjibAddress") {
        Ok(a) => a,
        Err(_) =>  String::from("127.0.0.1:4014")
    }
}

pub(crate) fn mjib_db_address() -> String {
    match env::var("MjibDBAddress") {
        Ok(a) => a,
        Err(_) =>  String::from("127.0.0.1:8099")
    }
}

pub(crate) fn gateway_ip() -> String {
    match env::var("GATEWAY_IP") {
        Ok(a) => a,
        Err(_) =>  String::from("127.0.0.1")
    }
}

pub(crate) fn mjib_db_root_user() -> String {
    match env::var("SURREALDB_ROOT_USER") {
        Ok(a) => a,
        Err(_) =>  String::from("root_user")
    }
}

pub(crate) fn mjib_db_root_pw() -> String {
    match env::var("SURREALDB_ROOT_PASSWORD") {
        Ok(a) => a,
        Err(_) =>  String::from("root_pw")
    }
}

pub(crate) fn mjib_db_ns() -> String {
    match env::var("Mjib_DB_NS") {
        Ok(a) => a,
        Err(_) =>  String::from("ns")
    }
}

pub(crate) fn mjib_db_db() -> String {
    match env::var("Mjib_DB_DB") {
        Ok(a) => a,
        Err(_) =>  String::from("db")
    }
}

pub(crate) fn mjib_access_method() -> String {
    match env::var("Mjib_ACCESS_METHOD") {
        Ok(a) => a,
        Err(_) =>  String::from("user")
    }
}
/*
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
    */