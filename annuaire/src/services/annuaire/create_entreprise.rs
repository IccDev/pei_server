use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Entreprise
}};
use crate::services::DatabaseService;

pub async fn create_entreprise(entreprise: Entreprise, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Entreprise>(entreprise).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
