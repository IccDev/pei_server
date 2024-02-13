use std::{collections::HashMap, convert::TryInto, iter::Iterator};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use argon2::{self, Config as Argon2Config};
use anyhow::Result;
use jsonwebtoken::{encode, TokenData, decode, Header, Validation, EncodingKey, DecodingKey};
use lazy_static::lazy_static;
use std::sync::Arc;
use tide::Request;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use async_std::stream;
use std::time::Duration;
use futures::StreamExt;
use tide_acme::{AcmeConfig, TideRustlsExt};
use mailchecker::is_valid;
use zxcvbn::zxcvbn;
use chbs::{config::BasicConfig, prelude::*};
use totp_rs::{Algorithm, TOTP};
extern crate biscuit_auth as biscuit;
use biscuit::{crypto::KeyPair, token::Biscuit};
use regex::Regex;

lazy_static! {
    static ref DB : Arc<rocksdb::DB> = {

        let prefix_extractor = rocksdb::SliceTransform::create_fixed_prefix(3);

        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.set_prefix_extractor(prefix_extractor);

        let configure = env_var_config();
        let db = rocksdb::DB::open(&opts, configure.db).unwrap();
        Arc::new(db)
    };
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnvVarConfig {
  pub port: u16,
  pub jwt_expiry: i64,
  pub origin: String,
  pub jwt_secret: String,
  pub db: String,
  pub secure: bool,
  pub certs: String,
  pub domain: String,
  pub admin_token: String,
  pub auto_cert: bool,
  pub key_path: String,
  pub cert_path: String,
  pub password_checker: bool,
  pub totp_duration: u64,
}



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub event: String,
    pub timestamp: i64,
    pub data: serde_json::Value,
    pub tenant_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventForm {
    pub event: String,
    pub data: serde_json::Value,
}

fn replace(key: String, value: Vec<u8>) -> Result<()> {
    DB.put(key.clone(), value.clone())?;
    Ok(())
}

fn soft_delete_user(username: String) -> Result<()> {
    match get_user_by_username(username)? {
        Some(mut user) => {
            user.revoked = true;
            puts_user(user)?;
            Ok(())
        },
        None => { Ok(()) }
    }
}

fn activate_user(username: String) -> Result<()> {
    match get_user_by_username(username)? {
        Some(mut user) => {
            user.revoked = false;
            puts_user(user)?;
            Ok(())
        },
        None => { Ok(()) }
    }
}

fn modify_user(update_user_form: UpdateUserForm) -> Result<Option<String>> {
    match get_user_by_username(update_user_form.clone().username)? {
        Some(mut user) => {
            
            match update_user_form.tenant_name {
                Some(tn) => {
                    user.tenant_name = tn;
                },
                None => {}
            }
            
            match update_user_form.email {
                Some(email) => {
                    if is_valid(&email) {
                        user.email = Some(email);
                    }
                },
                None => {}
            }
            
            user.data = update_user_form.data;
            user.scopes = update_user_form.scopes;
            user.two_factor = update_user_form.two_factor;

            match update_user_form.password {
                Some(password) => {

                    let configure = env_var_config();

                    if configure.password_checker {
                        let estimate = zxcvbn(&password, &[&user.username]).unwrap();
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
                            return Ok(Some(j));
                        }
                    }

                    let config = Argon2Config::default();
                    let uuid_string = Uuid::new_v4().to_string();
                    let salt =  uuid_string.as_bytes();
                    let password = password.as_bytes();
                    let hashed = argon2::hash_encoded(password, salt, &config).unwrap();
                    user.password = hashed;
                },
                None => {}
            }
            puts_user(user)?;
            Ok(None)
        },
        None => { Ok(None) }
    }
}

fn get_user_by_username(user_username: String) -> Result<Option<User>> {
    let users = get_users()?;
    Ok(users.into_iter().filter(|user| user.username == user_username).last())
}

fn get_users() -> Result<Vec<User>> {
    let prefix = "users".to_string();
    let i = DB.prefix_iterator(prefix.as_bytes());
    let res : Vec<User> = i.map(|(_, v)| {
        let data: User = rmp_serde::from_read_ref(&v).unwrap();
        data
    }).collect();
    Ok(res)
}

fn puts_user(user: User) -> Result<()> {
    let key = format!("users_{}", user.username);
    let value = rmp_serde::to_vec_named(&user)?;
    replace(key, value)?;
    Ok(())
}

fn get_events(tenant_name: Option<String>) -> Result<Vec<Event>> {
    
    let prefix: String;

    match tenant_name {
        Some(tn) => {
            prefix = format!("events_{}", tn);
        },
        None => {
            prefix = format!("events");
        }
    }

    let i = DB.prefix_iterator(prefix.as_bytes());
    let res : Vec<Event> = i.map(|(_, v)| {
        let data: Event = rmp_serde::from_read_ref(&v).unwrap();
        data
    }).collect();
    Ok(res)
}

fn puts_event(event: Event) -> Result<()> {
    let key = format!("events_{}_{}", event.tenant_name, event.event);
    let value = rmp_serde::to_vec_named(&event)?;
    replace(key, value)?;
    Ok(())
}

fn jwt_aud(scopes: Vec<String>, exp: i64, username: String) -> Result<Option<String>> {
    let biscuit_root = KeyPair::new();
    let biscuit_public_key = biscuit_root.public();
    let public_key_bytes = biscuit_public_key.to_bytes();

    let mut builder = Biscuit::builder(&biscuit_root);

    for scope in scopes.clone() {
        let mut parts = scope.split(":");
        let first = parts.next().unwrap_or_else(|| "INTERNAL_ERROR");
        let second = parts.next().unwrap_or_else(|| "INTERNAL_ERROR");
        if first == "INTERNAL_ERROR" || second == "INTERNAL_ERROR" {
            return Ok(None);
        }
        let f = format!("right(#authority, \"{}\", #{})", first, second);
        let t = f.as_ref();
        builder.add_authority_fact(t)?;
    }
    
    let biscuit = builder.build()?;
    Ok(Some(json!({"key": public_key_bytes, "token": biscuit.to_vec()?, "expiry": exp, "username": username, "scopes": scopes}).to_string()))
}


async fn create_jwt(login: LoginForm) -> Result<Option<String>> {

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
}

fn env_var_config() -> EnvVarConfig {
 
    let mut port : u16 = 8080;
    let mut jwt_expiry : i64 = 86400;
    let mut secure = false;
    let mut auto_cert = true;
    let mut origin = "*".to_string();
    let mut jwt_secret = "secret".to_string();
    let mut db: String = "db".to_string();
    let mut certs = "certs".to_string();
    let mut domain = "localhost".to_string();
    let mut admin_token = "letmein".to_string();
    let mut key_path = "certs/private_key.pem".to_string();
    let mut cert_path = "certs/chain.pem".to_string();
    let mut password_checker = false;
    let mut totp_duration: u64 = 300;
    let _ : Vec<String> = go_flag::parse(|flags| {
        flags.add_flag("port", &mut port);
        flags.add_flag("origin", &mut origin);
        flags.add_flag("jwt_expiry", &mut jwt_expiry);
        flags.add_flag("jwt_secret", &mut jwt_secret);
        flags.add_flag("secure", &mut secure);
        flags.add_flag("db", &mut db);
        flags.add_flag("domain", &mut domain);
        flags.add_flag("certs", &mut certs);
        flags.add_flag("admin_token", &mut admin_token);
        flags.add_flag("auto_cert", &mut auto_cert);
        flags.add_flag("key_path", &mut key_path);
        flags.add_flag("cert_path", &mut cert_path);
        flags.add_flag("password_checker", &mut password_checker);
        flags.add_flag("totp_duration", &mut totp_duration);
    });

    EnvVarConfig{port, origin, jwt_expiry, jwt_secret, secure, domain, certs, db, admin_token, auto_cert, key_path, cert_path, password_checker, totp_duration}
}

async fn jwt_verify(token: String) -> Result<Option<TokenData<Claims>>> {

    let configure = env_var_config();

    let mut parts = token.split(" ");
    let auth_type = parts.next().unwrap();
    if auth_type == "Bearer" {
        let token = parts.next().unwrap();
        let _ = match decode::<Claims>(&token,  &DecodingKey::from_secret(configure.jwt_secret.as_ref()), &Validation::default()) {
            Ok(c) => { return Ok(Some(c)); },
            Err(_) => { return Ok(None); }
        };
    } else if auth_type == "Basic" {
        let basic_encoded = parts.next().unwrap();

        let _ = match base64::decode(basic_encoded) {
            Ok(c) => {
                let _ = match std::str::from_utf8(&c) {
                    Ok(basic) => {
                        let mut basic_parts = basic.split(":");
                        let user_name = basic_parts.next().unwrap();
                        let password = basic_parts.next().unwrap();
                        let user_value = get_user_by_username(user_name.to_string())?;
                        match user_value {
                            Some(user) => {
                                if argon2::verify_encoded(&user.password, password.as_ref())? {
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
                                    let my_token = TokenData{
                                        header: Header::default(),
                                        claims: my_claims,
                                    };
                                    return Ok(Some(my_token));
                                }
                            },
                            None => { return Ok(None); }
                        }
                    },
                    Err(_) => { return Ok(None); }
                };
            },
            Err(_) => { return Ok(None); }
        };
    }
    Ok(None)
}

async fn insert(user: User, event_form: EventForm) -> Result<()> {
  
    let timestamp = nippy::get_unix_ntp_time().await?;
    let id = uuid::Uuid::new_v4();

    let event = Event{
        id,
        data: event_form.data, 
        event: event_form.event, 
        user_id: user.id, 
        timestamp,
        tenant_name: user.tenant_name,
    };

    puts_event(event.clone())?;
    Ok(())
}





async fn insert_event(mut req: Request<()>) -> tide::Result {
    let token_value = req.header("authorization");
    match token_value {
        Some(token_header) => {
            let token = token_header.last().to_string();
            let jwt_value = jwt_verify(token).await?;
            match jwt_value {
                Some(jwt) => {
                    let r =  req.body_string().await?;
                    let event_form : EventForm = serde_json::from_str(&r)?;
                    match get_user_by_username(jwt.claims.sub)? {
                        Some(user) => {
                            insert(user, event_form).await?;
                            Ok(tide::Response::builder(200).header("content-type", "application/json").build())
                        },
                        None => { Ok(tide::Response::builder(401).header("content-type", "application/json").build()) }
                    }
                },
                None => { Ok(tide::Response::builder(401).header("content-type", "application/json").build()) }
            }
        },
        None => { Ok(tide::Response::builder(401).header("content-type", "application/json").build()) }
    }
}

async fn verify_user(req: Request<()>) -> tide::Result {
    let token_value = req.header("authorization");
    match token_value {
        Some(token_header) => {
            let token = token_header.last().to_string();
            let jwt_value = jwt_verify(token).await?;
            match jwt_value {
                Some(jwt) => {
                    let username = jwt.claims.sub;
                    match get_user_by_username(username.clone())? {
                        Some(_) => {
                            let aud = jwt.claims.aud;
                            Ok(tide::Response::builder(200).body(aud.clone()).header("content-type", "application/json").build())
                        },
                        None => {
                            Ok(tide::Response::builder(401).header("content-type", "application/json").build())
                        }
                    }
                },
                None => { Ok(tide::Response::builder(401).header("content-type", "application/json").build()) }
            }
        },
        None => { Ok(tide::Response::builder(401).header("content-type", "application/json").build()) }
    }
}

async fn get_user(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let admin_token_form : AdminTokenForm = serde_json::from_str(&r)?;
    let configure = env_var_config();
    if configure.admin_token == admin_token_form.admin_token {
        let users = get_users()?;
        let users: Vec<_> = users.iter().map(|user| {
            let mut u = user.to_owned();
            u.password = "***".to_string(); 
            u.totp = "***".to_string();
            u
        }).collect();
        Ok(tide::Response::builder(200).body(json!(users)).header("content-type", "application/json").build())
    } else {
        Ok(tide::Response::builder(401).header("content-type", "application/json").build())
    }
}

async fn list_users(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let admin_token_form : AdminTokenForm = serde_json::from_str(&r)?;
    let configure = env_var_config();
    if configure.admin_token == admin_token_form.admin_token {
        let users = get_users()?;
        let users: Vec<_> = users.iter().map(|user| {
            let mut u = user.to_owned();
            u.password = "***".to_string(); 
            u.totp = "***".to_string();
            u
        }).collect();
        Ok(tide::Response::builder(200).body(json!(users)).header("content-type", "application/json").build())
    } else {
        Ok(tide::Response::builder(401).header("content-type", "application/json").build())
    }
}

async fn revoke_user(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let revoke_user_form : RevokeUserForm = serde_json::from_str(&r)?;
    let configure = env_var_config();
    if configure.admin_token == revoke_user_form.admin_token {
        soft_delete_user(revoke_user_form.username)?;
        Ok(tide::Response::builder(200).header("content-type", "application/json").build())
    } else {
        Ok(tide::Response::builder(401).header("content-type", "application/json").build())
    }
}

async fn unrevoke_user(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let revoke_user_form : RevokeUserForm = serde_json::from_str(&r)?;
    let configure = env_var_config();
    if configure.admin_token == revoke_user_form.admin_token {
        activate_user(revoke_user_form.username)?;
        Ok(tide::Response::builder(200).header("content-type", "application/json").build())
    } else {
        Ok(tide::Response::builder(401).header("content-type", "application/json").build())
    }
}

async fn update_user(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let update_user_form : UpdateUserForm = serde_json::from_str(&r)?;
    let configure = env_var_config();
    if configure.admin_token == update_user_form.admin_token {
        match modify_user(update_user_form)? {
            Some(err) => {
                Ok(tide::Response::builder(400).body(err).header("content-type", "application/json").build())
            },
            None => {
                Ok(tide::Response::builder(200).header("content-type", "application/json").build())
            }
        }

    } else {
        Ok(tide::Response::builder(401).header("content-type", "application/json").build())
    }
}

async fn health(_: Request<()>) -> tide::Result {
    Ok(tide::Response::builder(200).header("content-type", "application/json").build())
}

async fn create_qr(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let create_qr_form : CreateQRForm = serde_json::from_str(&r)?;

    let configure = env_var_config();

    if create_qr_form.admin_token == configure.admin_token {
        match get_user_by_username(create_qr_form.username)? {
            Some(user) => {
                let totp = TOTP::new(
                    Algorithm::SHA512,
                    6,
                    1,
                    30,
                    user.totp,
                );
                let code = totp.get_qr(&user.username, &create_qr_form.issuer).unwrap();
                let j = json!({"qr": code});
            
                Ok(tide::Response::builder(200).body(j).header("content-type", "application/json").build())
            },
            None => {
                Ok(tide::Response::builder(401).header("content-type", "application/json").build())
            }
        }
    } else {
        Ok(tide::Response::builder(401).header("content-type", "application/json").build())
    }
}

async fn create_totp(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let create_totp_form : CreateTOTPForm = serde_json::from_str(&r)?;

    let configure = env_var_config();

    if create_totp_form.admin_token == configure.admin_token {
        match get_user_by_username(create_totp_form.username)? {
            Some(user) => {
                let totp = TOTP::new(
                    Algorithm::SHA512,
                    6,
                    1,
                    configure.totp_duration,
                    user.totp,
                );

                let time = nippy::get_unix_ntp_time().await?;
                let token = totp.generate(time.try_into()?);
                let j = json!({"totp": token});
            
                Ok(tide::Response::builder(200).body(j).header("content-type", "application/json").build())
            },
            None => {
                Ok(tide::Response::builder(401).header("content-type", "application/json").build())
            }
        }
    } else {
        Ok(tide::Response::builder(401).header("content-type", "application/json").build())
    }
}

async fn password_reset(mut req: Request<()>) -> tide::Result {
    let r =  req.body_string().await?;
    let password_reset_form : PasswordResetForm = serde_json::from_str(&r)?;

    let configure = env_var_config();

    match get_user_by_username(password_reset_form.username)? {
        Some(user) => {
            let totp = TOTP::new(
                Algorithm::SHA512,
                6,
                1,
                configure.totp_duration,
                user.totp,
            );

            let time = nippy::get_unix_ntp_time().await?;
            let check = totp.check(&password_reset_form.totp, time.try_into()?);

            if check {
                let update_user_form = UpdateUserForm{
                    username: user.username,
                    password: Some(password_reset_form.password),
                    tenant_name: Some(user.tenant_name),
                    admin_token: configure.admin_token,
                    email: user.email,
                    data: user.data,
                    scopes: user.scopes,
                    two_factor: user.two_factor,
                };
                modify_user(update_user_form)?;
                Ok(tide::Response::builder(200).header("content-type", "application/json").build())
            } else {
                Ok(tide::Response::builder(401).header("content-type", "application/json").build())  
            }
        },
        None => {
            Ok(tide::Response::builder(401).header("content-type", "application/json").build())
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    let configure = env_var_config();

    let cors = CorsMiddleware::new()
    .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    .allow_headers("authorization".parse::<HeaderValue>().unwrap())
    .allow_origin(Origin::from(configure.origin))
    .allow_credentials(false);
    
    let mut app = tide::new();
    app.with(driftwood::DevLogger);
    app.with(cors);
    app.at("/").get(health);
    app.at("/").head(health);
    app.at("/insert").post(insert_event);
    app.at("/create_user").post(create_user);
    app.at("/login").post(login_user);
    app.at("/verify").get(verify_user);
    app.at("/list_users").post(list_users);
    app.at("/revoke_user").post(revoke_user);
    app.at("/get_user").post(get_user);
    app.at("/unrevoke_user").post(unrevoke_user);
    app.at("/update_user").post(update_user);
    app.at("/create_qr").post(create_qr);
    app.at("/create_totp").post(create_totp);
    app.at("/password_reset").post(password_reset);

    app.at("/sse").get(tide::sse::endpoint(|req: Request<()>, sender| async move {

        let token_value = req.header("authorization");
        match token_value {
            Some(token_header) => {
                let token = token_header.last().to_string();
                let jwt_value = jwt_verify(token).await?;
                match jwt_value {
                    Some(jwt) => {
                        let user_value = get_user_by_username(jwt.claims.sub)?;

                        match user_value {
                            Some(user) => {

                                let mut cache: HashMap<String, Event> = HashMap::new();
                        
                                let mut interval = stream::interval(Duration::from_millis(100));
                                while let Some(_) = interval.next().await {
                                    let events = get_events(None)?;

                                    for evt in events {
                                        if evt.tenant_name == user.tenant_name {
                                            if !cache.contains_key(&evt.event) {
                                                let id = uuid::Uuid::new_v4();
                                                sender.send(&evt.event, evt.data.to_string(), Some(&id.to_string())).await?;
                                                cache.insert(evt.event.clone(), evt.clone());
                                            } else {
                                                let value_maybe = cache.get_key_value(&evt.event);
                                                match value_maybe {
                                                    Some((_, v)) => {
                                                        let current_data = evt.data.to_string();
                                                        let stored_data = v.data.to_string();
                                                
                                                        if current_data != stored_data {
                                                            let id = uuid::Uuid::new_v4();
                                                            sender.send(&evt.event, evt.data.to_string(), Some(&id.to_string())).await?;
                                                            cache.insert(evt.event.clone(), evt.clone());    
                                                        }
                                                    },
                                                    None => { return Ok(()); }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            None => { return Ok(()); }
                        }
                        Ok(())
                    },
                    None => { Ok(()) }
                }
            },
            None => { Ok(()) }
        }
    }));

    let ip = format!("0.0.0.0:{}", configure.port);

    if configure.secure && configure.auto_cert {
        app.listen(
            tide_rustls::TlsListener::build().addrs("0.0.0.0:443").acme(
                AcmeConfig::new()
                    .domains(vec![configure.domain])
                    .cache_dir(configure.certs)
                    .production(),
            ),
        )
        .await?;
    } else if configure.secure && !configure.auto_cert {
        app.listen(
            tide_rustls::TlsListener::build()
            .addrs("0.0.0.0:443")
            .cert(configure.cert_path)
            .key(configure.key_path)
        )
        .await?;
    } else {
        app.listen(ip).await?;
    }

    Ok(())
}