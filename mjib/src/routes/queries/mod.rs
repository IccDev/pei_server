use warp::reply::Response;
use warp::Reply;

pub fn get_sessions() -> impl Reply {
    Response::new(format!("{:#?}", DB.get_sessions()).into())
}

pub fn get_session_by_id(id: i32) -> impl Reply {
    Response::new(format!("{:#?}", DB.get_session_by_id(id)).into())
}