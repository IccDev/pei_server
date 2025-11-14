use serde::{Serialize, Deserialize};
use crate::models::{Course, Discipline};


#[derive(Queryable, Selectable, Associations, Insertable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::courses_disciplines)]
#[diesel(belongs_to(crate::models::Course))]
#[diesel(belongs_to(crate::models::Discipline))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(course_id, discipline_id))]
pub struct CourseDiscipline {
    pub course_id: i32,
    pub discipline_id: i32
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateCourseDiscipline {
    pub course_id: i32,
    pub discipline_ids: Vec<i32>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseDisciplineData {
    pub course: Course,
    pub disciplines: Vec<Discipline>
}