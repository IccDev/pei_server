use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::models::Section;


#[derive(Queryable, AsChangeset, Selectable, Associations, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i32,
    pub identifier: Uuid,
    pub last_name: String, // nom
    pub first_name: String, // prenom
    pub email: String,
    pub age: i32,
    pub created_at: NaiveDateTime
}


#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateUser {
    pub last_name: String, // nom
    pub first_name: String, // prenom
    pub email: String,
    pub age: i32,
}

/*
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DisciplineData {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub section: Section,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}*/