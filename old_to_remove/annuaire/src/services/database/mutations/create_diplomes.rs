use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        DiplomeCertificat, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<DiplomeCertificat> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: DiplomeCertificat, _system: &ServiceAssistant<Self>) -> Self::Response {
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.create_diplome(&message).await)))
    }
}
