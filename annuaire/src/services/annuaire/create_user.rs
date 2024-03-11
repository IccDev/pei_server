use common::acteur::Acteur;
use inter_services_messages::{ResponseData, annuaire::User};
use crate::services::DatabaseService;


pub async fn create_user(start: User, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, User>(start).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
