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
use inter_services_messages::{users::User, ResponseData, UserResponseData};

#[derive(Debug, Clone)]
pub struct ListUsersMsg;


#[async_trait::async_trait]
impl Serve<ListUsersMsg> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, _message: ListUsersMsg, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.list_users().await
    }
}


impl DatabaseService {
    pub(crate) async fn list_users(&self) -> Result<ResponseData, String> {

        match sqlx::query_as::<_, User>(format!("SELECT * FROM icc.users;").as_str())
            .fetch_all(&self.pool)
            .await 
        {
            Ok(res) => Ok(ResponseData::User(UserResponseData::ListUsers(res))),
            Err(_) => {
                // need to check well before sending the error message back
                Err("not able to register!".to_owned())
            }
        }
    }
}