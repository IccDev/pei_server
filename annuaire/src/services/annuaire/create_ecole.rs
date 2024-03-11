use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Ecole
}};
use crate::services::DatabaseService;

pub async fn create_ecole(ecole: Ecole, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Ecole>(ecole).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
