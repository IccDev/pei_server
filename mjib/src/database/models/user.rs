use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::{RecordId, value::Datetime},
    chrono::offset::Utc
};
use crate::database::models::{CityData, RoleRow, ChurchData};



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserRow {
    pub id: RecordId,
    pub email: String,
    pub password: String,
    pub role_id: RecordId,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture_url: String,
    pub church_id: RecordId,
    pub phone: String,
    pub is_active: bool,
    pub last_login: Datetime,
    pub created_at: Datetime,
    pub updated_at: Datetime
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct User {
    pub email: String,
    pub password: String,
    pub role_id: RecordId,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture_url: String,
    pub church_id: RecordId,
    pub phone: String,
    pub is_active: bool,
    pub last_login: Datetime,
    pub created_at: Datetime,
    pub updated_at: Datetime
}


impl std::default::Default for User {
    fn default() -> Self { 
        User {
            email: String::with_capacity(0),
            password: String::with_capacity(0),
            role_id: RecordId::from(("role", 123)),
            first_name: String::with_capacity(0),
            last_name: String::with_capacity(0),
            profile_picture_url: String::with_capacity(0),
            church_id: RecordId::from(("church", 123)),
            phone: String::with_capacity(0),
            is_active: false,
            last_login: Datetime::default(),
            created_at: Datetime::default(),
            updated_at: Datetime::default()
        }
     }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserInput {
    pub email: String,
    pub password: String,
    pub role_id: RecordId,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture_url: String,
    pub church_id: RecordId,
    pub phone: String,
    pub is_active: bool
}

impl std::default::Default for UserInput {
    fn default() -> Self { 
        UserInput {
            email: String::with_capacity(0),
            password: String::with_capacity(0),
            role_id: RecordId::from(("role", 123)),
            first_name: String::with_capacity(0),
            last_name: String::with_capacity(0),
            profile_picture_url: String::with_capacity(0),
            church_id: RecordId::from(("church", 123)),
            phone: String::with_capacity(0),
            is_active: false
        }
     }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserData {
    pub id: RecordId,
    pub email: String,
    pub role: RoleRow,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture_url: String,
    pub church: ChurchData,
    pub phone: String,
    pub is_active: bool,
    pub last_login: Datetime,
    pub created_at: Datetime,
    pub updated_at: Datetime
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "self::serde")]
pub struct AuthParams {
    email: String,
    password: String,
}

impl From<UserInput> for User {
    fn from(value: UserInput) -> User {
        User {
            email: value.email.to_owned(),
            password: value.password.to_owned(),
            role_id: value.role_id.to_owned(),
            first_name: value.first_name.to_owned(),
            last_name: value.last_name.to_owned(),
            profile_picture_url: value.profile_picture_url.to_owned(),
            church_id: value.church_id.to_owned(),
            phone: value.phone.to_owned(),
            is_active: value.is_active.to_owned(),
            last_login: Datetime::from(Utc::now()),
            created_at: Datetime::from(Utc::now()),
            updated_at: Datetime::from(Utc::now())
        }
    }
}