use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, match_request::Params, read_post_body};
use crate::DB;
use crate::database::models::AnneeAcademique;

pub(crate) async fn create_annee_academique_route(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let data = read_post_body::<Vec<AnneeAcademique>>(req).await;

    DB.create_annee_academique(data).await
}
