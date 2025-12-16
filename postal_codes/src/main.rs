use std::net::IpAddr;
use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use postal_codes::DB;
use postal_codes::routes::routes_handlers;
use postal_codes::database::Database;


#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    DB.initiate();
    let ip = env::var("POSTAL_CODE_IP").map_or("192.168.60.22".to_string(), |ip| ip);
    let port = env::var("POSTAL_CODE_PORT").map_or(4045, |p| p.parse::<u16>().expect("port is u16"));
    println!("listen on url: http:{ip}:{port}");

    warp::serve(routes_handlers())
        .run((IpAddr::from_str(&ip).expect("No an IpAddr"), port))
        .await;
}