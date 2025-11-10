use warp::reply::Response;
use warp::Reply;
use crate::models::{
    CreateSection, UpdateSection, CreateDiscipline, UpdateDiscipline, 
    CreateCourse, UpdateCourse, CreateCourseDiscipline
};
use crate::DB;
use crate::database::Database;

pub fn create_section(section: CreateSection) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.create_section(section, &mut db)).into())
}

pub fn update_section(section: UpdateSection) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.update_section(section, &mut db)).into())
}

pub fn create_discipline(discipline: CreateDiscipline) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.create_discipline(discipline, &mut db)).into())
}

pub fn update_discipline(discipline: UpdateDiscipline) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.update_discipline(discipline, &mut db)).into())
}

pub fn create_course(course: CreateCourse) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.create_course(course, &mut db)).into())
}

pub fn update_course(course: UpdateCourse) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.update_course(course, &mut db)).into())
}

pub fn create_course_disciplines(course_discipline: CreateCourseDiscipline) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.create_course_disciplines(course_discipline, &mut db)).into())
}