use crate::models::{CreateAccount, Account, Login, Logout};
use std::env;
use serde_json::Value;


pub fn get_repository_id() {

}

pub fn create_auth_user(repository_id: i32, email: String, password: String) -> Result<String, ureq::Error> {
    let auth_host: String = env::var("IDENTIFIER_HOST").expect("not able to load db_url from .env");
    let create_account_url = format!("{auth_host}/create/account");
    
    let account = CreateAccount {
        identifier: email,
        identifier_type: "Email".to_string(),
        password,
        repository_id
    };

    let recv_body = ureq::post(&create_account_url)
        .send_json(&account)?
        .body_mut()
        .read_to_string()?;

    let get_account_url = format!("{auth_host}/account/{recv_body}");
    let mut body = ureq::get(&get_account_url)
        .call()?
        .body_mut()
        .read_to_string()?;
    let Ok(raw_data): serde_json::Result<Value> = serde_json::from_str(&body) else {
            return Ok("Err".to_string());
    };

    let Some(raw_Str) = raw_data.as_str() else {
        return Ok("Err".to_string());
    };

    let Ok(account): serde_json::Result<Account> = serde_json::from_str(raw_Str) else {
            return Ok("Err".to_string());
    };

    Ok(account.identifier_id.to_string())
}

pub fn login_auth_user(login: Login) -> Result<String, ureq::Error> {
    let auth_host: String = env::var("IDENTIFIER_HOST").expect("not able to load db_url from .env");
    let url = format!("{auth_host}/login");

    ureq::post(&url)
        .send_json(&login)?
        .body_mut()
        .read_to_string()
}

pub fn logout_auth_user(logout: Logout) -> Result<String, ureq::Error> {
    let auth_host: String = env::var("IDENTIFIER_HOST").expect("not able to load db_url from .env");
    let url = format!("{auth_host}/logout");

    ureq::post(&url)
        .send_json(&logout)?
        .body_mut()
        .read_to_string()
}