use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Competence
}};
use crate::services::DatabaseService;

pub async fn create_competence(competence: Competence, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Competence>(competence).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
