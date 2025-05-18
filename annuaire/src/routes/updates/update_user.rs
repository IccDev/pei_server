use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::err;
use crate::router::{BoxedBody, match_request::Params, read_post_body};
use crate::DB;
use crate::database::models::RegisterFormData;

pub(crate) async fn update_user_route(req: Request<IncomingBody>, params: Params) -> Response<BoxedBody> {
    match params.get("user_id") {
        Some(user_id) => {
            let data = read_post_body::<RegisterFormData>(req).await;
            DB.update_user(user_id, data).await
        },
        None => err("No user found!")
    }
    
}
