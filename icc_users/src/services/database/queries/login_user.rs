use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{LoginForm, User, ResponseData};



#[async_trait::async_trait]
impl Serve<LoginForm> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: LoginForm, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.login_user(message).await
    }
}


impl DatabaseService {
    pub(crate) async fn login_user(&self, msg: LoginForm) -> Result<ResponseData, String> {

        
        match &self.get_user_by_email(&msg.email).await 
        {
            Ok(user) => create_jwt(user, msg).await,
            Err(e) => {
                // need to check well before sending the error message back
                Err(e.to_string())
            }
        }
    }
}


async fn create_jwt(user: &User, login: LoginForm) -> Result<ResponseData, String> {

    println!("usser: {:#?}", user);
    println!("login: {:#?}", login);
    Err("not able to login!".to_owned())
    /*
    match get_user_by_username(login.username)? {
        Some(user) => {
            if !user.revoked {
                let verified = argon2::verify_encoded(&user.password, login.password.as_bytes())?;
                if verified {
                    match user.two_factor {
                        Some(two_factor) => {
                            if two_factor {
                                match login.totp {
                                    Some(token) => {
                                        let totp = TOTP::new(
                                            Algorithm::SHA512,
                                            6,
                                            1,
                                            30,
                                            user.clone().totp,
                                        );
                                        let time = nippy::get_unix_ntp_time().await?;
                                        if totp.check(&token, time.try_into()?) {
                                            let app = env_var_config();
                                            let iat = nippy::get_unix_ntp_time().await?;
                                            let exp = iat + app.jwt_expiry;
                                            let iss = "Broker".to_string();
                                            let aud: String;
                                            match user.scopes.clone() {
                                                Some(scopes) => {
                                                    match jwt_aud(scopes, exp, user.clone().username)? {
                                                        Some(a) => {
                                                            aud = a;
                                                        },
                                                        None => { aud = "".to_string() }
                                                    }
                                                },
                                                None => { aud = "".to_string() }
                                            }
                                            let my_claims = Claims{sub: user.clone().username, exp, iat, iss, aud};
                                            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(app.jwt_secret.as_ref()))?;
                                            Ok(Some(token))
                                        } else {
                                            Ok(None)
                                        }
                                    },
                                    None => { Ok(None) }
                                }
                            } else {
                                let app = env_var_config();
                                let iat = nippy::get_unix_ntp_time().await?;
                                let exp = iat + app.jwt_expiry;
                                let iss = "Broker".to_string();
                                let aud: String;
                                match user.scopes.clone() {
                                    Some(scopes) => {
                                        match jwt_aud(scopes, exp, user.clone().username)? {
                                            Some(a) => {
                                                aud = a;
                                            },
                                            None => { aud = "".to_string() }
                                        }
                                    },
                                    None => { aud = "".to_string() }
                                }
                                let my_claims = Claims{sub: user.clone().username, exp, iat, iss, aud};
                                let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(app.jwt_secret.as_ref())).unwrap();
                                Ok(Some(token))                
                            }
                        },
                        None => {
                            let app = env_var_config();
                            let iat = nippy::get_unix_ntp_time().await?;
                            let exp = iat + app.jwt_expiry;
                            let iss = "Broker".to_string();
                            let aud: String;
                            match user.scopes.clone() {
                                Some(scopes) => {
                                    match jwt_aud(scopes, exp, user.clone().username)? {
                                        Some(a) => {
                                            aud = a;
                                        },
                                        None => { aud = "".to_string() }
                                    }
                                },
                                None => { aud = "".to_string() }
                            }
                            let my_claims = Claims{sub: user.clone().username, exp, iat, iss, aud};
                            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(app.jwt_secret.as_ref()))?;
                            Ok(Some(token))
                        }
                    }
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        },
        None => { Ok(None) }
    }
    */
}