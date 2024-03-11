use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, Departement
}};
use crate::services::DatabaseService;

pub async fn create_departement(departement: Departement, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, Departement>(departement).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
