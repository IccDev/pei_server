use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{Role, RoleRow}
};



impl DBService {
    pub async fn create_roles(&self, roles: Vec<Role>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<RoleRow>>("role").content(roles).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}