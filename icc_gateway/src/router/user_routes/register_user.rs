use icc_common::{
    hyper::{Request, Response, Body, body::to_bytes, StatusCode},
    match_request::Params
};
use inter_services_messages::{MessageData, RegisterUser};
use serde_json::json;
use super::user_data::UserRegisterForm;
use crate::clients::user_client;


pub(crate) async fn not_user_route(_req: Request<Body>, _params: Params) -> Response<Body> {
    return Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("content-type", "application/json")
        .body(Body::from(json!({"error": "Route Not Found!"}).to_string()))
        .unwrap();
}

pub(crate) async fn register_user(req: Request<Body>, _params: Params) -> Response<Body> {
    let Ok(bytes) = to_bytes(req.into_body()).await else {
        return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json!({"error": "Unable to convert body into bytes"}).to_string()))
                .unwrap();
    };

    let body_str = String::from_utf8_lossy(&bytes).into_owned();
    let user_form : UserRegisterForm = match serde_json::from_str(&body_str) {
        Ok(r) => r,
        Err(e) => {
            return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("content-type", "application/json")
                    .body(Body::from(json!({"error": format!("{:#?}", e)}).to_string()))
                    .unwrap()
        }
    };

    let two_factor = match user_form.two_factor {
        Some(fact) => fact,
        None => false
    };

    let data = MessageData::RegisterUser(RegisterUser {
        last_name: user_form.last_name.clone(),
        first_name: user_form.first_name.clone(),
        password: user_form.password.clone(),
        user_token: user_form.user_token.clone(),
        email: user_form.email.clone(),
        two_factor
    });
    
    match user_client::client(data).await {
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
        },
    }
}
