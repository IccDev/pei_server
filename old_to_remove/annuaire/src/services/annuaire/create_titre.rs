use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Titre
}};
use crate::services::DatabaseService;

pub async fn create_titre(titre: Titre, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Titre>(titre).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
