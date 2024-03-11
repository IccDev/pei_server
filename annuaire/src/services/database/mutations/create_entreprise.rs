use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        Entreprise, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<Entreprise> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: Entreprise, _system: &ServiceAssistant<Self>) -> Self::Response {
        let entreprise_locale = message.clone().localite.unwrap_or_default();
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.save_entreprise(&message, &entreprise_locale).await)))
    }
}
