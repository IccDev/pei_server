use icc_common::acteur::Acteur;
use std::env;
use inter_services_messages::ResponseData;
use serde_json::json;
use crate::services::database::{DatabaseService, ListBilletsMsg};


pub async fn list_billets(user_token: String, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, ListBilletsMsg>(ListBilletsMsg(user_token)).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
