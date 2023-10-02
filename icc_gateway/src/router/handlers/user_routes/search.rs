use icc_common::{
    hyper::{Request, Response, Body},
    match_request::Params, 
    url::form_urlencoded
};
use inter_services_messages::{MessageData, users::{AnnuaireSearch, AnnuaireSearchResponse}, ResponseData, UserMessageData, UserResponseData};
use serde_json::json;
use crate::{
    clients::user_client,
    router::responses::{ok, bad_request}
};
use std::collections::HashMap;


pub(crate) async fn handle_search(req: Request<Body>, _params: Params) -> Response<Body> {
    
    let query_params: HashMap<String, String> = req
                .uri()
                .query()
                .map(|v| {
                    form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(HashMap::new);
    
    let data = MessageData::User(UserMessageData::Annuaire(AnnuaireSearch {
        key: query_params.get("key").cloned(),
        church: query_params.get("church").cloned()
    }));

    match user_client::client(data).await {
        Ok(ResponseData::User(UserResponseData::Annuaire(AnnuaireSearchResponse {stars: star_data}))) => ok(Body::from(json!({"data": star_data}).to_string())),
        Ok(_) => bad_request("Something Happen!"),
        Err(err) => bad_request(&format!("{:#?}", err))
    }
}
