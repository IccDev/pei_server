use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::User;

impl DatabaseService {
    pub(crate) async fn get_user_by_email(&self, email: &str) -> Result<User, String> {
        match sqlx::query_as::<_, User>(format!("SELECT * FROM icc.users WHERE email = '{}';", email).as_str())
            .fetch_one(&self.pool)
            .await 
        {
            Ok(res) => Ok(res),
            Err(e) => {
                // need to check well before sending the error message back
                Err("not able to get user by email!".to_owned())
            }
        }
    }
}