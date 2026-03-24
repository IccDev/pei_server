
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Section {
    pub id: i32,
    pub name: String,
    pub comment: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub academic_year_id: i32
}



#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateSection {
    pub name: String,
    pub comment: String,
    pub academic_year_id: i32
}

#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct UpdateSection {
    pub id: i32,
    pub name: String,
    pub comment: String,
    pub academic_year_id: i32
}