use warp::reply::Response;
use warp::Reply;
use crate::DB;
use crate::database::Database;

pub fn get_sections() -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_sections(&mut db)).into())
}

pub fn get_section_by_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_section_by_id(id, &mut db)).into())
}

pub fn get_discipline_by_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_discipline_by_id(id, &mut db)).into())
}

pub fn get_discipline_by_section_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_discipline_by_section_id(id, &mut db)).into())
}

pub fn get_disciplines() -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_disciplines(&mut db)).into())
}

pub fn get_courses() -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_courses(&mut db)).into())
}

pub fn get_course_by_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_course_by_id(id, &mut db)).into())
}

pub fn get_course_disciplines_by_course_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_course_disciplines_by_course_id(id, &mut db)).into())
}

pub fn get_course_disciplines() -> impl Reply {
    let mut db = DB.access();
    Response::new(format!("{:#?}", DB.get_course_disciplines(&mut db)).into())
}