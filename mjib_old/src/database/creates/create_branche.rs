use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{Branche, BrancheRow}
};



impl DBService {
    pub async fn create_branches(&self, branches: Vec<Branche>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<BrancheRow>>("branche").content(branches).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}