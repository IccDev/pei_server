use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{UserBranches, UserBranchesRow}
};



impl DBService {
    pub async fn create_user_branches(&self, user_branches: Vec<UserBranches>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<UserBranchesRow>>("user_branches").content(user_branches).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}