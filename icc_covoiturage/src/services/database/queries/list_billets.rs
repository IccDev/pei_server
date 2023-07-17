use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait,
    sqlx
};
use inter_services_messages::{covoiturage::Billet, ResponseData, CovoiturageResponseData};

#[derive(Debug, Clone)]
pub struct ListBilletsMsg(pub String);


#[async_trait::async_trait]
impl Serve<ListBilletsMsg> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, _message: ListBilletsMsg, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.list_billets().await
    }
}


impl DatabaseService {
    pub(crate) async fn list_billets(&self) -> Result<ResponseData, String> {

        match sqlx::query_as::<_, Billet>(format!("SELECT * FROM icc.billets;").as_str())
            .fetch_all(&self.pool)
            .await 
        {
            Ok(res) => Ok(ResponseData::Covoiturage(CovoiturageResponseData::ListBillets(res))),
            Err(er) => {
                // need to check well before sending the error message back
                Err("not able to get billets!".to_owned())
            }
        }
    }
}