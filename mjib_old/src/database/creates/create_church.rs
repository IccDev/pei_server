use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{Church, ChurchRow}
};



impl DBService {
    pub async fn create_churches(&self, churches: Vec<Church>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<ChurchRow>>("church").content(churches).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}