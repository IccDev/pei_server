use warp::reply::Response;
use warp::Reply;
use crate::models::{
    CreateSection, UpdateSection, CreateDiscipline, UpdateDiscipline, 
    CreateCourse, UpdateCourse, CreateCourseDiscipline, CreateAge, UpdateAge
};
use crate::DB;
use crate::database::Database;

pub fn create_section(section: CreateSection) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_section(section, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_section(section: UpdateSection) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_section(section, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_discipline(discipline: CreateDiscipline) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_discipline(discipline, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_discipline(discipline: UpdateDiscipline) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_discipline(discipline, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_course(course: CreateCourse) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_course(course, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_course(course: UpdateCourse) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_course(course, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn create_course_disciplines(course_discipline: CreateCourseDiscipline) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_course_disciplines(course_discipline, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

// User operations: inscription, login, logout



// age api: create, update, select
pub fn create_age(age: CreateAge) -> impl Reply {
    let mut db = DB.access();
    let res = DB.create_age(age, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}

pub fn update_age(update_age: UpdateAge) -> impl Reply {
    let mut db = DB.access();
    let res = DB.update_age(update_age, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}