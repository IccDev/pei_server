use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{ResponseData, users::LoginForm};



#[async_trait::async_trait]
impl Serve<LoginForm> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: LoginForm, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.login_user(message).await
    }
}


impl DatabaseService {
    pub(crate) async fn login_user(&self, msg: LoginForm) -> Result<ResponseData, String> {

        println!("login: {:#?}", msg);
        Err("not able to login!".to_owned())
    }
}