use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        Domaine, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<Domaine> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: Domaine, _system: &ServiceAssistant<Self>) -> Self::Response {
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.create_domaine(&message).await)))
    }
}
