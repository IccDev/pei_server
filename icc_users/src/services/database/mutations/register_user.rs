use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait,
    sqlx,
    time::OffsetDateTime
};
use inter_services_messages::{User, ResponseData};


#[async_trait::async_trait]
impl Serve<User> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: User, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.register_user(message).await
    }
}


impl DatabaseService {
    pub(crate) async fn register_user(&self, message: User) -> Result<ResponseData, String> {
        let created_at = match message.created_at {
            Some(d) => d,
            None => OffsetDateTime::now_utc()
        };

        match sqlx::query(
            format!(
                r#"INSERT INTO icc.users ( 
                    email,
                    id,
                    activated,
                    last_name,
                    first_name,
                    password,
                    is_admin,
                    totp,
                    two_factor,
                    created_at
                )
                VALUES ( '{}', '{}', {}, '{}', '{}', '{}', {}, '{}', {}, '{}' );
                "#,
                    &message.email,
                    &message.id,
                    &message.activated,
                    &message.last_name,
                    &message.first_name,
                    &message.password,
                    &message.is_admin,
                    &message.totp,
                    &message.two_factor,
                    &created_at
    
            ).as_str())
            .execute(&self.pool)
            .await 
        {
            Ok(_) => Ok(ResponseData::RegisterUser),
            Err(_) => {
                // need to check well before sending the error message back
                Err("not able to register!".to_owned())
            }
        }
    }
}