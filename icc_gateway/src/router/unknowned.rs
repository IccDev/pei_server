use icc_common::{
    match_request::Params,
    hyper::{Request, Response, Body, StatusCode},
};
use serde_json::json;



pub(crate) async fn unknowed_route(_req: Request<Body>, _params: Params) -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "application/json")
        .body(Body::from(json!({"error": "Route Not Found!"}).to_string()))
        .unwrap()
}