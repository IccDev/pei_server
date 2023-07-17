use icc_common::{
    hyper::{Request, Response, Body, StatusCode},
    match_request::Params
};
use serde_json::json;
use crate::{
    clients::covoiturage_client,
    router::response::{ok, err}
};
use inter_services_messages::{MessageData, ResponseData, CovoiturageMessageData, CovoiturageResponseData};



pub(crate) async fn get_billets(_req: Request<Body>, _params: Params) -> Response<Body> {
    let data = MessageData::Covoiturage(CovoiturageMessageData::ListBillets(String::from("djedou.icc@gmail.com")));
    match covoiturage_client::client(data).await {
        Ok(ResponseData::Covoiturage(CovoiturageResponseData::ListBillets(res))) => {
                ok(Body::from(json!({"data": res}).to_string()))
        },
        Ok(_) => {
            err("Something Happen!")
        },
        Err(er) => {
            let error = format!("{:#?}", er);
            err(&error)
        }
    }
}
