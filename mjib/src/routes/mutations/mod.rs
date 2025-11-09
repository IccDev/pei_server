use warp::reply::Response;
use warp::Reply;
use crate::models::{CreateSection, UpdateSection, CreateDiscipline, UpdateDiscipline};
use crate::DB;
use crate::database::Database;

pub fn create_section(section: CreateSection) -> impl Reply {
    Response::new(format!("{:#?}", DB.create_section(section)).into())
}

pub fn update_section(section: UpdateSection) -> impl Reply {
    Response::new(format!("{:#?}", DB.update_section(section)).into())
}

pub fn create_discipline(discipline: CreateDiscipline) -> impl Reply {
    Response::new(format!("{:#?}", DB.create_discipline(discipline)).into())
}

pub fn update_discipline(discipline: UpdateDiscipline) -> impl Reply {
    Response::new(format!("{:#?}", DB.update_discipline(discipline)).into())
}