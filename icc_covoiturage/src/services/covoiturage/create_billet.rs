use icc_common::{
    uuid::Uuid,
    acteur::Acteur,
    time::OffsetDateTime
};
use std::env;
use inter_services_messages::{covoiturage::Billet, ResponseData};
use serde_json::json;
use crate::services::database::{DatabaseService};


pub async fn create_billet(billet: Billet, acteur: Acteur) -> Result<ResponseData, String> {
    //TODO: check if the user exist before further operations
    match acteur.call_service::<DatabaseService, Billet>(billet).await {
        Ok(res) => res,
        Err(e) => {
            Err(e.to_string())
        }
    }
}
