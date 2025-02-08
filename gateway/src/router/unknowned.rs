use super::match_request::Params;




pub(crate) fn unknowed_route(_params: Params) -> Result<String, String> {
    Err("Unknowed route".to_string())
}

/*
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



pub(crate) fn err_msg(msg: String) -> Response<BoxedBody> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .body(BoxedBody::new(msg))
        .unwrap()
}*/