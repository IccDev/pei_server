pub mod queries;

use self::queries::*;

use warp::{Filter, Reply};



pub fn routes_handlers() -> impl Filter<Extract = impl Reply> + Clone {
    warp::get()
        .and(warp::path!("search" / String).map(get_postal_codes))
}