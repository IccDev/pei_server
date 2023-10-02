use icc_common::{
    hyper::{Request, Response, Body, body::to_bytes},
    match_request::Params
};
use inter_services_messages::{MessageData, users::LoginForm, ResponseData, UserMessageData, UserResponseData};
use crate::{
    clients::user_client,
    router::responses::{ok, err, bad_request}
};


pub(crate) async fn handle_login(req: Request<Body>, _params: Params) -> Response<Body> {
    
    let Ok(body_bytes) = to_bytes(req.into_body()).await else {
        return err("Unable to convert body into bytes");
    };

    let body_str = String::from_utf8_lossy(&body_bytes).into_owned();
    
    let data = MessageData::User(UserMessageData::LoginUser(LoginForm(body_str)));
    match user_client::client(data).await {
        Ok(ResponseData::User(UserResponseData::LoginUser(jwt))) => ok(Body::from(jwt)),
        Ok(_) => bad_request("Something Happen!"),
        Err(err) => bad_request(&format!("{:#?}", err))
    }
}
