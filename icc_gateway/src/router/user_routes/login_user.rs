use icc_common::{
    hyper::{Request, Response, Body, body::to_bytes, StatusCode},
    match_request::Params
};
use inter_services_messages::{MessageData, LoginForm, ResponseData};
use serde_json::json;
use crate::clients::user_client;


pub(crate) async fn login_user(req: Request<Body>, _params: Params) -> Response<Body> {
    let Ok(bytes) = to_bytes(req.into_body()).await else {
        return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json!({"error": "Unable to convert body into bytes"}).to_string()))
                .unwrap();
    };

    let body_str = String::from_utf8_lossy(&bytes).into_owned();
    let login_form: LoginForm = match serde_json::from_str(&body_str) {
        Ok(r) => r,
        Err(e) => {
            return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("content-type", "application/json")
                    .body(Body::from(json!({"error": format!("{:#?}", e)}).to_string()))
                    .unwrap()
        }
    };


    let data = MessageData::LoginUser(login_form);
    match user_client::client(data).await {
        Ok(ResponseData::LoginUser(jwt)) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json!({"data": jwt}).to_string()))
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

    /*match user_client::login_user(login_form).await {
        Some(jwt) => {
            let msg = json!({"jwt": jwt}).to_string();
            Ok(tide::Response::builder(200).body(msg).header("content-type", "application/json").build())
        },
        None => {
            Ok(tide::Response::builder(401).header("content-type", "application/json").build())
        }
    }*/
}
