use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::age)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Age {
    pub id: i32,
    pub max: i32,
    pub min: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::age)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateAge {
    pub max: i32,
    pub min: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct UpdateAge {
    pub id: i32,
    pub max: i32,
    pub min: i32,
}