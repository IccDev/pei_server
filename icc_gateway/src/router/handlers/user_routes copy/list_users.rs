use icc_common::{
    hyper::{Request, Response, Body, StatusCode},
    match_request::Params
};
use serde_json::json;
use crate::clients::user_client;
use inter_services_messages::{MessageData, ResponseData, UserResponseData, UserMessageData};



pub(crate) async fn list_users(_req: Request<Body>, _params: Params) -> Response<Body> {
    let data = MessageData::User(UserMessageData::ListUsers(String::from("mfjlsdjflsjflqjsldjflqjsljdf")));
    match user_client::client(data).await {
        Ok(ResponseData::User(UserResponseData::ListUsers(res))) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json!({"data": res}).to_string()))
                .unwrap()
        },
        Ok(_) => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json!({"error": "Something Happen!"}).to_string()))
                .unwrap()
        },
        Err(err) => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json!({"error": format!("{:#?}", err)}).to_string()))
                .unwrap()
        }
    }
}
