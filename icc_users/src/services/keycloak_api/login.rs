use crate::{
    services::{
        keycloak_api::KeycloakAPIService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait,
    http_client::{Request, h1::H1Client, HttpClient}
};
use inter_services_messages::{ResponseData, users::LoginForm, UserResponseData};
use std::env;



#[async_trait::async_trait]
impl Serve<LoginForm> for KeycloakAPIService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: LoginForm, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.login_user(message).await
    }
}


impl KeycloakAPIService {
    pub(crate) async fn login_user(&self, LoginForm(msg): LoginForm) -> Result<ResponseData, String> {
        let address = match env::var("KeycloakAddress") {
            Ok(a) => a,
            Err(_) => String::from("http://192.168.1.5:8180") // 192.168.1.5 is this pc ip_address
        };

        let client = H1Client::new();
        let mut req = Request::post(format!("{address}/realms/icc_bruxelles/protocol/openid-connect/token").as_str());
        req.append_header("Content-Type", "application/x-www-form-urlencoded");
        req.set_body(msg);

        match client.send(req).await {
            Ok(mut res) => {
                let Ok(res_string) = res.body_string().await else {
                    return Err("No able to convert keycloak response into string!".to_owned())
                };

                Ok(ResponseData::User(UserResponseData::LoginUser(res_string)))
            },
            Err(_) => {
                Err("connection to keycloack failled!".to_owned())
            }
        }
    }
}