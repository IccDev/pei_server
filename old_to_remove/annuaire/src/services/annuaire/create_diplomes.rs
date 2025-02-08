use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, DiplomeCertificat
}};
use crate::services::DatabaseService;

pub async fn create_diplome(diplome: DiplomeCertificat, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, DiplomeCertificat>(diplome).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
