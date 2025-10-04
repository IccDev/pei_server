use warp::reply::Response;
use warp::Reply;

pub fn create_session(session: CreateSession) -> impl Reply {
    Response::new(format!("{:#?}", DB.create_session(session)).into())
}