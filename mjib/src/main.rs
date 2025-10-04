use std::net::IpAddr;
use std::env;
use std::str::FromStr;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    DB.initiate();
    let ip = env::var("MJIB_IP").map_or("127.0.0.1".to_string(), |ip| ip);
    println!("http:{ip}:4014");

    warp::server(routes_handlers())
        .run((IpAddr::from_str(&ip).expect("No an IpAddr"), 4014))
}
