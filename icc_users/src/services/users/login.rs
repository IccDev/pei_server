use icc_common::acteur::Acteur;
use inter_services_messages::{ResponseData, users::LoginForm};
use crate::services::KeycloakAPIService;


pub async fn login(login_form: LoginForm, acteur: Acteur) -> Result<ResponseData, String> {
    
    match acteur.call_service::<KeycloakAPIService, LoginForm>(login_form).await {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}
