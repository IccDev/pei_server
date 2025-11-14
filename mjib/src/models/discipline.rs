use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::models::Section;


#[derive(Queryable, AsChangeset, Selectable, Associations, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::disciplines)]
#[diesel(belongs_to(crate::models::Section))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Discipline {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub section_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::disciplines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateDiscipline {
    pub name: String,
    pub comment: Option<String>,
    pub section_id: i32
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct UpdateDiscipline {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub section_id: i32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DisciplineData {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub section: Section,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}