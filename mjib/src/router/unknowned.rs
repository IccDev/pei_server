use common_crates::{
    hyper::{Request, Response, body::Incoming as IncomingBody, StatusCode},
    serde_json::json
};
use super::{BoxedBody, match_request::Params};

pub(crate) async fn unknowed_route(_req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(BoxedBody::new(json!({"error": "Route Not Found!"}).to_string()))
        .unwrap()
}