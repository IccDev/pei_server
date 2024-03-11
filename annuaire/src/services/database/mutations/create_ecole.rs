use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        Ecole, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<Ecole> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: Ecole, _system: &ServiceAssistant<Self>) -> Self::Response {
        let ecole_locale = message.clone().localite.unwrap_or_default();
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.save_ecole(&message, &ecole_locale).await)))
    }
}
