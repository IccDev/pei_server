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
        .or(warp::path!("get" / "course" / "disciplines").map(get_course_disciplines))
        .or(warp::path!("get" / "course" / i32 / "disciplines").map(get_course_disciplines_by_course_id))
        .or(warp::path!("get" / "age").map(get_age))
        .or(warp::path!("get" / "users").map(get_users))
        .or(warp::path!("get" / "user"/ i32 / "section").map(get_user_sections_by_user_id))
        .or(warp::path!("get" / "user" / i32).map(get_user_by_id))
        .or(warp::path!("get" / "academic_years").map(get_academic_years))
    .or(
    warp::post()
        .and(warp::path!("create"/ "section").and(warp::body::json()).map(create_section))
        .or(warp::path!("update"/ "section").and(warp::body::json()).map(update_section))
        .or(warp::path!("create"/ "discipline").and(warp::body::json()).map(create_discipline))
        .or(warp::path!("update"/ "discipline").and(warp::body::json()).map(update_discipline))
        .or(warp::path!("create"/ "course").and(warp::body::json()).map(create_course))
        .or(warp::path!("update"/ "course").and(warp::body::json()).map(update_course))
        .or(warp::path!("create"/ "course" / "disciplines").and(warp::body::json()).map(create_course_disciplines))
        .or(warp::path!("create"/ "age").and(warp::body::json()).map(create_age))
        .or(warp::path!("update"/ "age").and(warp::body::json()).map(update_age))
        .or(warp::path!("create"/ "user").and(warp::body::json()).map(create_user))
        .or(warp::path!("create"/ "user"/"admin").and(warp::body::json()).map(create_admin_user))
        .or(warp::path!("login").and(warp::body::json()).map(login))
        .or(warp::path!("logout").and(warp::body::json()).map(logout))
        .or(warp::path!("create"/ "user" / "section").and(warp::body::json()).map(create_user_section))
        .or(warp::path!("create"/ "academic_year").and(warp::body::json()).map(create_academic_year))
        .or(warp::path!("update"/ "academic_year").and(warp::body::json()).map(update_academic_year))
    )
}