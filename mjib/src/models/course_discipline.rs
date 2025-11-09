use chrono::NaiveDateTime;
use serde::{Deserialize};
use crate::models::Section;


#[derive(Queryable, AsChangeset, Selectable, Associations, Insertable, Debug, Deserialize, Clone)]
#[diesel(table_name = crate::schema::courses)]
#[diesel(belongs_to(crate::models::Section))]
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

/*
#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Clone)]
#[diesel(table_name = crate::schema::disciplines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateDiscipline {
    pub name: String,
    pub comment: Option<String>,
    pub section_id: i32
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct UpdateDiscipline {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub section_id: i32
}

#[derive(Debug, Deserialize, Clone)]
pub struct DisciplineData {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub section: Section,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
*/