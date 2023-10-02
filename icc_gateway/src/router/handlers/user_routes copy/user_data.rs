use serde_derive::{Serialize, Deserialize};
/*
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserForm {
    pub last_name: String,
    pub first_name: String,
    pub password: String,
    //pub tenant_name: String,
    pub admin_token: String,
    pub email: String,
    //pub data: Option<serde_json::Value>,
    pub scopes: Option<Vec<String>>,
    pub two_factor: Option<bool>
}*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRegisterForm {
    pub last_name: String,
    pub first_name: String,
    pub password: String,
    pub user_token: String,
    pub email: String,
    pub two_factor: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserForm {
    pub last_name: String,
    pub first_name: String,
    pub user_token: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub two_factor: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminTokenForm {
    pub admin_token: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateQRForm {
    pub issuer: String,
    pub admin_token: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTOTPForm {
    pub admin_token: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordResetForm {
    pub totp: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevokeUserForm {
    pub admin_token: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,          
    pub iat: i64,         
    pub iss: String,         
    pub sub: String,
    pub aud: String,
}