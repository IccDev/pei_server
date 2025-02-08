mod eglises;
mod get_infos_to_create_user;

pub use eglises::*;
pub use get_infos_to_create_user::*;

use common_crates::{
    //tokio,
    serde_json, 
    http_body_util::BodyExt,
    serde::de::Error,
    hyper::body::Buf
};

use common_crates::{
    hyper::{Request, body::Incoming as IncomingBody}
};



pub async fn get_request_body<T>(req: Request<IncomingBody>) -> std::result::Result<T, serde_json::Error> 
where T: for<'de> common_crates::serde::Deserialize<'de>
{
    let Ok(whole_body) = req.collect().await else {
        return Err(serde_json::Error::custom("".to_string()));
    };
    serde_json::from_reader(whole_body.aggregate().reader())
}