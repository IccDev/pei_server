use warp::reply::Response;
use warp::Reply;
use crate::DB;
use crate::database::Database;

pub fn get_sections() -> impl Reply {
    Response::new(format!("{:#?}", DB.get_sections()).into())
}

pub fn get_section_by_id(id: i32) -> impl Reply {
    Response::new(format!("{:#?}", DB.get_section_by_id(id)).into())
}

pub fn get_discipline_by_id(id: i32) -> impl Reply {
    Response::new(format!("{:#?}", DB.get_discipline_by_id(id)).into())
}

pub fn get_discipline_by_section_id(id: i32) -> impl Reply {
    Response::new(format!("{:#?}", DB.get_discipline_by_section_id(id)).into())
}

pub fn get_disciplines() -> impl Reply {
    Response::new(format!("{:#?}", DB.get_disciplines()).into())
}