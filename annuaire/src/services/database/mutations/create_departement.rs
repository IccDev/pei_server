use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        Departement, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<Departement> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: Departement, _system: &ServiceAssistant<Self>) -> Self::Response {
        let id_campus = &message.id_campus.clone().unwrap_or_default();
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.create_departement(id_campus.to_owned(), &message).await)))
    }
}
