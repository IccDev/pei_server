use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, match_request::Params, read_post_body};
use crate::DB;
use crate::database::models::AuthParams;

pub(crate) async fn signin_route(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let data = read_post_body::<AuthParams>(req).await;
    DB.signin(data).await
}