use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Campus
}};
use crate::services::DatabaseService;

pub async fn create_campus(campus: Campus, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Campus>(campus).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
