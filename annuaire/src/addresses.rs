use std::env;
use std::net::SocketAddr;
use std::str::FromStr;



pub(crate) fn annuaire_address() -> String {
    match env::var("AnnuaireAddress") {
        Ok(a) => a,
        Err(_) =>  String::from("127.0.0.1:4013")
    }
}

pub(crate) fn annuaire_db_address() -> String {
    match env::var("AnnuaireDatabaseAddress") {
        Ok(a) => a,
        Err(_) =>  String::from("84.234.16.224:8199")
    }
}

pub(crate) fn gateway_ip() -> String {
    match env::var("GATEWAY_IP") {
        Ok(a) => a,
        Err(_) =>  String::from("127.0.0.1")
    }
}

pub(crate) fn annuaire_db_root_user() -> String {
    match env::var("SURREALDB_ROOT_USER") {
        Ok(a) => a,
        Err(_) =>  String::from("root_user")
    }
}

pub(crate) fn annuaire_db_root_pw() -> String {
    match env::var("SURREALDB_ROOT_PASSWORD") {
        Ok(a) => a,
        Err(_) =>  String::from("root_pw")
    }
}

pub(crate) fn annuaire_db_ns() -> String {
    match env::var("ANNUAIRE_DB_NS") {
        Ok(a) => a,
        Err(_) =>  String::from("ns")
    }
}

pub(crate) fn annuaire_db_db() -> String {
    match env::var("ANNUAIRE_DB_DB") {
        Ok(a) => a,
        Err(_) =>  String::from("db")
    }
}