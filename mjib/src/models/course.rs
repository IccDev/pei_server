use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::courses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub video_link: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}


#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::courses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateCourse {
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub video_link: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct UpdateCourse {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub video_link: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseData {
    pub id: i32,
    pub name: String,
    pub comment: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub video_link: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}