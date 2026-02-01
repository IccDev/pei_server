use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::academic_year)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct AcademicYear {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::academic_year)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateAcademicYear {
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct UpdateAcademicYear {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime
}