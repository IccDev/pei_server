
use chrono::NaiveDateTime;
use serde::Deserialize;


#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Clone)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Section {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}



#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Clone)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateSection {
    pub name: String,
    pub comment: Option<String>,
}

#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Clone)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct UpdateSection {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>
}