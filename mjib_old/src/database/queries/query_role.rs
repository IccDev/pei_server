use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::RoleRow
};



impl DBService {
    pub async fn query_roles(&self) -> Response<BoxedBody> {
        match DB.db.select::<Vec<RoleRow>>("role").await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}