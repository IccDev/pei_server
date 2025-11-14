use warp::reply::Response;
use warp::Reply;
use crate::DB;
use crate::database::Database;

pub fn get_sections() -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_sections(&mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_section_by_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_section_by_id(id, &mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_discipline_by_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_discipline_by_id(id, &mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_discipline_by_section_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_discipline_by_section_id(id, &mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_disciplines() -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_disciplines(&mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_courses() -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_courses(&mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_course_by_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_course_by_id(id, &mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_course_disciplines_by_course_id(id: i32) -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_course_disciplines_by_course_id(id, &mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}

pub fn get_course_disciplines() -> impl Reply {
    let mut db = DB.access();
    Response::new(DB.get_course_disciplines(&mut db).map_or("".to_string(), |d| format!("{d:#?}")).into())
}