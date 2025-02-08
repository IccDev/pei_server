use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Specialite
}};
use crate::services::DatabaseService;

pub async fn create_specialite(specialite: Specialite, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Specialite>(specialite).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
