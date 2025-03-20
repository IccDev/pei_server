use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, err, match_request::Params, read_post_body};
use crate::DB;

pub(crate) async fn query_users_route(req: Request<IncomingBody>, params: Params) -> Response<BoxedBody> {
    match params.get("key") {
        Some(key) => {
            match params.get("church") {
                Some(church) => {
                    DB.query_users(key.to_owned(), church.to_owned()).await
                },
                None => {
                    err("no key and church are provided!")
                }
            } 
        },
        None => {
            err("no key provided!")
        }
    }  
}