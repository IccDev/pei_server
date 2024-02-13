use icc_common::{
    hyper::{Request, Response, Body, body::to_bytes, StatusCode},
    match_request::Params
};
use inter_services_messages::{MessageData, ResponseData, CovoiturageResponseData, CovoiturageMessageData, covoiturage::Billet};
use serde_json::json;
use crate::{
    clients::covoiturage_client,
    router::response::{ok, err},
};

pub(crate) async fn create_billet(req: Request<Body>, _params: Params) -> Response<Body> {
    let Ok(bytes) = to_bytes(req.into_body()).await else {
        return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("content-type", "application/json")
        .body(Body::from(json!({"error": "Unable to convert body into bytes"}).to_string()))
        .unwrap();
    };
    
    let body_str = String::from_utf8_lossy(&bytes).into_owned();
    
    let billet: Billet = match serde_json::from_str(&body_str) {
        Ok(r) => r,
        Err(er) => {
            let e = format!("{:#?}", er);
            return err(&e)
        }
    };

    let data = MessageData::Covoiturage(CovoiturageMessageData::CreateBillet(billet));
    
    match covoiturage_client::client(data).await {
        Ok(ResponseData::Covoiturage(CovoiturageResponseData::CreateBillet(created_id))) => {
                ok(Body::from(json!({"data": created_id}).to_string()))
        },
        Ok(_) => {
                err("Something Happen!")
        },
        Err(er) => {
                let e = format!("{:#?}", er);
                err(&e)
        }
    }
}
