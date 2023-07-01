use icc_common::acteur::Acteur;
use std::env;
use inter_services_messages::ResponseData;
use serde_json::json;
use crate::services::database::{DatabaseService, ListUsersMsg};


pub async fn list_users(user_token: String, acteur: Acteur) -> Result<ResponseData, String> {
    let anonymous_token = match env::var("AnonymousToken") {
        Ok(a) => a,
        Err(_) => String::from("mfjlsdjflsjflqjsldjflqjsljdf")
    };

    if &anonymous_token == &user_token {
        match acteur.call_service::<DatabaseService, ListUsersMsg>(ListUsersMsg).await {
            Ok(res) => res,
            Err(e) => Err(e.to_string())
        }
        
    } else {
        let j = json!({"error": "anonymous_token is incorrect"}).to_string();
        return Err(j);
    }
}
