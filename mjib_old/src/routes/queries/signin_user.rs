use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, match_request::Params, read_post_body, err};
use crate::DB;
use crate::database::models::AuthParams;

pub(crate) async fn signin_user_route(req: Request<IncomingBody>, params: Params) -> Response<BoxedBody> {
    match params.get("user_id") {
        Some(user_id) => DB.signin_user(user_id.to_owned()).await,
        None => err("No user found!")
    }
}