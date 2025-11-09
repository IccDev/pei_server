pub mod mutations;
pub mod queries;


use self::mutations::*;
use self::queries::*;

use warp::{Filter, Reply};



pub fn routes_handlers() -> impl Filter<Extract = impl Reply> + Clone {
    warp::get()
        .and(warp::path!("get" / "section" / "all").map(get_sections))
        .or(warp::path!("get" / "section" / i32).map(get_section_by_id))
        .or(warp::path!("get" / "discipline" / "all").map(get_disciplines))
        .or(warp::path!("get" / "discipline" / i32).map(get_discipline_by_id))
        .or(warp::path!("get" / "discipline" / "section" / i32).map(get_discipline_by_section_id))
        .or(warp::path!("get" / "course" / "all").map(get_courses))
        .or(warp::path!("get" / "course" / i32).map(get_course_by_id))
    .or(
    warp::post()
        .and(warp::path!("create"/ "section").and(warp::body::json()).map(create_section))
        .or(warp::path!("update"/ "section").and(warp::body::json()).map(update_section))
        .or(warp::path!("create"/ "discipline").and(warp::body::json()).map(create_discipline))
        .or(warp::path!("update"/ "discipline").and(warp::body::json()).map(update_discipline))
        .or(warp::path!("create"/ "course").and(warp::body::json()).map(create_course))
        .or(warp::path!("update"/ "course").and(warp::body::json()).map(update_course))
    )
}