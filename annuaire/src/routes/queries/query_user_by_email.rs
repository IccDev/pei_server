use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, match_request::Params, read_post_body, err};
use crate::DB;
use crate::database::models::Email;


pub(crate) async fn query_user_by_email_route(req: Request<IncomingBody>, params: Params) -> Response<BoxedBody> {
    let data = read_post_body::<Email>(req).await;
    DB.query_user_by_email(data.email).await
}