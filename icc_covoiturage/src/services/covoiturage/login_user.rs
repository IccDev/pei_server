use icc_common::acteur::Acteur;
use inter_services_messages::{ResponseData, LoginForm};
use crate::services::database::DatabaseService;


pub async fn login_user(login_form: LoginForm, acteur: Acteur) -> Result<ResponseData, String> {
    match acteur.call_service::<DatabaseService, LoginForm>(login_form).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
