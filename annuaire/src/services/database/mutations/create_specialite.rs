use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        Specialite, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<Specialite> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: Specialite, _system: &ServiceAssistant<Self>) -> Self::Response {
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.create_specialite(&message).await)))
    }
}
