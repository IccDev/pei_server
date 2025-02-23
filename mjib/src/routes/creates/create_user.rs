use common_crates::{
    hyper::{Request, Response, body::{Incoming as IncomingBody, Body}},
    http_body_util::BodyExt
};
use crate::router::{BoxedBody, match_request::Params, read_post_body};
use crate::DB;
use crate::database::models::{User, UserInput};

pub(crate) async fn signup_route(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let data = read_post_body::<UserInput>(req).await;
    DB.signup(User::from(data)).await
}
