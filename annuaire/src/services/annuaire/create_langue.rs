use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Langue
}};
use crate::services::DatabaseService;

pub async fn create_langue(langue: Langue, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Langue>(langue).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
