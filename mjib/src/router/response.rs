use common_crates::{
    hyper::{body::Bytes, Request, Response, StatusCode, body::{Buf, Incoming as IncomingBody}},
    serde_json::{json, from_reader},
    http_body_util::{BodyExt, combinators::BoxBody},
    serde::de,
    tracing::error
};
use std::convert::Infallible;



pub type BoxedBody = BoxBody<Bytes, Infallible>;

pub(crate) fn ok(body: BoxedBody) -> Response<BoxedBody> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(body)
        .unwrap()
}

pub(crate) fn err(msg: &str) -> Response<BoxedBody> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(BoxedBody::new(json!({"error": msg}).to_string()))
        .unwrap()
}


pub(crate) async fn read_post_body<T>(req: Request<IncomingBody>) -> T 
where T: Default + de::DeserializeOwned
{
    match req.collect().await {
        Ok(collect_body) => {
            // Aggregate the body...
            let whole_body = collect_body.aggregate();
            match from_reader(whole_body.reader()) {
                Ok(res) => res,
                Err(e) => {
                    error!("{e:#?}");
                    T::default()
                }
            }
        },
        Err(e) => {
            error!("{e:#?}");
            T::default()
        }
    }
}