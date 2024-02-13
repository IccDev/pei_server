use icc_common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::AnnuaireSearchInput};
use crate::services::DatabaseService;


pub async fn search_stars(search: AnnuaireSearchInput, acteur: Acteur) -> Result<ResponseData, String> {
    
    match acteur.call_service::<DatabaseService, AnnuaireSearchInput>(search).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
