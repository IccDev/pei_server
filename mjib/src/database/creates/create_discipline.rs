use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{Discipline, DisciplineRow}
};



impl DBService {
    pub async fn create_disciplines(&self, disciplines: Vec<Discipline>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<DisciplineRow>>("discipline").content(disciplines).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}