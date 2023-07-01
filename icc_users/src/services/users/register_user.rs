use icc_common::{
    mailchecker::is_valid,
    zxcvbn::zxcvbn,
    chbs::{config::BasicConfig, prelude::*},
    uuid::Uuid,
    argon2::{self, Config as Argon2Config},
    acteur::Acteur,
    time::OffsetDateTime
};
use std::env;
use inter_services_messages::{RegisterUser, User, ResponseData};
use serde_json::json;
use crate::services::database::{DatabaseService};


pub async fn create_user(user_form: RegisterUser, acteur: Acteur) -> Result<ResponseData, String> {
    let anonymous_token = match env::var("AnonymousToken") {
        Ok(a) => a,
        Err(_) => String::from("mfjlsdjflsjflqjsldjflqjsljdf")
    };

    if &anonymous_token == &user_form.user_token {
        if !is_valid(&user_form.email) {
            let j = json!({"error": "email is invalid"}).to_string();
            return Err(j);
        }

        if !is_user_unique(&user_form.email) {
            let j = json!({"error": "email already exist"}).to_string();
            return Err(j);
        } else {
            let estimate = zxcvbn(&user_form.password, &[&user_form.first_name, &user_form.last_name]).unwrap();
            if estimate.score() < 3 {
                let err: String;
                match estimate.feedback() {
                    Some(feedback) => {
                        match feedback.warning() {
                            Some(warning) => {
                                err = format!("password is too weak because {}", warning);
                            },
                            None => {
                                err = format!("password is too weak");
                            }
                        }
                    },
                    None => {
                        err = format!("password is too weak");
                    }
                }
                let j = json!({"error": err}).to_string();
                return Err(j);
            }

            let mut config = BasicConfig::default();
            config.words = 12;
            config.separator = "-".into();
            let scheme = config.to_scheme();
            let totp = scheme.generate();

            let uuid = Uuid::new_v4();
            let config = Argon2Config::default();
            let uuid_string = Uuid::new_v4().to_string();
            let salt =  uuid_string.as_bytes();
            let password = user_form.password.as_bytes();
            let hashed = argon2::hash_encoded(password, salt, &config).unwrap();

            let new_user = User {
                id: uuid, 
                activated: false,
                email: user_form.email.clone(),
                last_name: user_form.last_name.clone(),
                first_name: user_form.first_name.clone(),
                password: hashed,
                is_admin: false,
                totp,
                two_factor: user_form.two_factor,
                created_at: Some(OffsetDateTime::now_utc())
            };
            
            match acteur.call_service::<DatabaseService, User>(new_user).await {
                Ok(res) => res,
                Err(e) => Err(e.to_string())
            }
        }
        
    } else {
        let j = json!({"error": "anonymous_token is incorrect"}).to_string();
        return Err(j);
    }
}


fn is_user_unique(_email: &str) -> bool {
    /*let users = get_users()?;
    for user in users {
        if user.username == user_username {
            return Ok(false);
        }
    }*/
    true
}