use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::models::Section;


#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i32,
    pub identifier: String,
    pub last_name: String, // nom
    pub first_name: String, // prenom
    pub email: String,
    pub date_of_birth: String,
    pub gsm: String,
    pub pays: String,
    pub ville: String,
    pub eglise: String,
    pub situation_professionnelle: String,
    pub commenaire: String,
    pub is_admin: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime
}


#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CreateUserIn {
    pub last_name: String, // nom
    pub first_name: String, // prenom
    pub email: String,
    pub password: String,
    pub date_of_birth: String,
    pub gsm: String,
    pub pays: String,
    pub ville: String,
    pub eglise: String,
    pub situation_professionnelle: String,
    pub commenaire: String,
    pub is_admin: bool
}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUser {
    pub identifier: String,
    pub last_name: String, // nom
    pub first_name: String, // prenom
    pub email: String,
    pub date_of_birth: String,
    pub gsm: String,
    pub pays: String,
    pub ville: String,
    pub eglise: String,
    pub situation_professionnelle: String,
    pub commenaire: String,
    pub is_admin: bool,
    pub is_deleted: bool
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Login {
    pub identifier_value: String, // email
    pub password: String
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Logout {
    pub identifier_value: String // email
}


#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CreateAccount {
    pub identifier: String,
    pub identifier_type: String,
    pub password: String,
    pub repository_id: i32
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: i32,
    pub identifier_id: String,
    pub repository_id: i32,
    pub deactivated: bool,
    pub password: Option<String>
}