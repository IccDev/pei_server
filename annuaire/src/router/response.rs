use common_crates::{
    hyper::{body::Bytes, Response, StatusCode},
    serde_json::json,
    http_body_util::combinators::BoxBody
};
use std::convert::Infallible;

pub type BoxedBody = BoxBody<Bytes, Infallible /*GenericError*/>;

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