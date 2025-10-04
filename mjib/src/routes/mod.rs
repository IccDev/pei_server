pub mod mutations;
pub mod queries;


use self::mutations::*;
use self::queries::*;

use warp::{Filter, Reply};



pub fn routes_handlers() -> impl Filter<Extract = impl Reply> + Clone {
    warp::get()
        .and(warp::path!("get" / "session" / "all").map(get_sessions))
        .and(warp::path!("get" / "session" / i32).map(get_session_by_id))
    .or(
    warp::post()
        .and(warp::path!("create"/ "session").map(warp::body::json()).map(create_session))
    )
}