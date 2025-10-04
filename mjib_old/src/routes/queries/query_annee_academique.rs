use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, match_request::Params, read_post_body};
use crate::DB;

pub(crate) async fn query_annee_academique_route(_req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    DB.query_annee_academique().await
}