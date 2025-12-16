use warp::reply::Response;
use warp::Reply;
use crate::DB;
use crate::database::Database;

pub fn get_postal_codes(search: String) -> impl Reply {
    let mut db = DB.access();
    let res = DB.get_postal_codes(search, &mut db);
    match res.is_ok() {
        true => {
            Response::new(format!("{:#?}", serde_json::json!(res.unwrap()).to_string()).into())
        }
        false => {
            Response::new("".to_owned().into())
        }
    }
}