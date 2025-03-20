use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, match_request::Params, read_post_body, err};
use crate::DB;

pub(crate) async fn query_user_to_contact_route(req: Request<IncomingBody>, params: Params) -> Response<BoxedBody> {
    match params.get("id") {
        Some(id) => {
            DB.query_user_to_contact(id.to_owned()).await
        },
        None => {
            err("no key provided!")
        }
    } 
}