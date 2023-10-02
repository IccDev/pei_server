use icc_common::acteur::Acteur;
use inter_services_messages::{ResponseData, users::AnnuaireSearch};
use crate::services::DatabaseService;


pub async fn search_stars(search: AnnuaireSearch, acteur: Acteur) -> Result<ResponseData, String> {
    
    match acteur.call_service::<DatabaseService, AnnuaireSearch>(search).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
