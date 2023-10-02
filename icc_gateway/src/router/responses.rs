use serde_json::json;
use icc_common::{
    match_request::Params,
    hyper::{Request, Response, Body, StatusCode},
};


pub(crate) fn ok(body: Body) -> Response<Body> {
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

pub(crate) fn bad_request(msg: &str) -> Response<Body> {
    Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json!({"error": msg}).to_string()))
                .unwrap()
}

/*
pub(crate) fn ok_js(body: Body) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/javascript")
        .header("Accept", "application/javascript")
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(body)
        .unwrap()
}

pub(crate) fn ok_wasm(body: Body) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/wasm")
        .header("Accept", "application/wasm")
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(body)
        .unwrap()
}
*/


pub(crate) fn err(msg: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(Body::from(json!({"error": msg}).to_string()))
        .unwrap()
}

pub(crate) async fn unknowed_route(_req: Request<Body>, _params: Params) -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "application/json")
        .body(Body::from(json!({"error": "Route Not Found!"}).to_string()))
        .unwrap()
}