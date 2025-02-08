use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        Campus, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<Campus> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: Campus, _system: &ServiceAssistant<Self>) -> Self::Response {
        let campus_locale = message.clone().localite.unwrap_or_default();
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.save_campus(&message, &campus_locale).await)))
    }
}
