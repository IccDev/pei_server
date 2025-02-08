use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{
    annuaire::{
        UserPlusInfos, AnnuaireResponse
    }, ResponseData
};

#[async_trait::async_trait] 
impl Serve<UserPlusInfos> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: UserPlusInfos, _system: &ServiceAssistant<Self>) -> Self::Response {
        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(self.save_user_plus_infos(&message).await)))
    }
}
