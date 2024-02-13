use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{ResponseData, annuaire::AnnuaireSearch};


#[async_trait::async_trait]
impl Serve<AnnuaireSearch> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: AnnuaireSearch, _system: &ServiceAssistant<Self>) -> Self::Response {
        match message {
            AnnuaireSearch::ByKey(key) => self.user_competences_by_key(key).await,
            AnnuaireSearch::ById(id) => self.user_competences_by_id(id).await
        }
    }
}

impl DatabaseService {
    pub(crate) async fn user_competences_by_key(&self, msg: String) -> Result<ResponseData, String> {
    
        Err(String::with_capacity(0))
    }

    pub(crate) async fn user_competences_by_id(&self, msg: i32) -> Result<ResponseData, String> {
    
        Err(String::with_capacity(0))
    }
}