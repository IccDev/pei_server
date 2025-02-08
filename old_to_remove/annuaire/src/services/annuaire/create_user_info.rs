use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::{
    AnnuaireSearchInput, UserPlusInfos
}};
use crate::services::DatabaseService;

pub async fn create_user_info(user_info: UserPlusInfos, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, UserPlusInfos>(user_info).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
