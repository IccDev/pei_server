use chrono::NaiveDateTime;
use serde::Deserialize;


#[derive(Queryable, AsChangeset, Selectable, Insertable, Debug, Deserialize, Clone)]
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

    /*
    disciplineIds: ["1"],
     */
}


#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Clone)]
#[diesel(table_name = crate::schema::courses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateCourse {
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub video_link: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct UpdateCourse {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub video_link: Option<String>
}