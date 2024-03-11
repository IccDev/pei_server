use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Localite
}};
use crate::services::DatabaseService;

pub async fn create_localite(localite: Localite, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Localite>(localite).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
