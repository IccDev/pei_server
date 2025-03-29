use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{Cours, CoursRow}
};



impl DBService {
    pub async fn create_cours(&self, cours: Vec<Cours>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<CoursRow>>("cours").content(cours).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}