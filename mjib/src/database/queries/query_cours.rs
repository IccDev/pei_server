use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::CoursRow
};



impl DBService {
    pub async fn query_cours(&self) -> Response<BoxedBody> {
        let query = r#"SELECT * FROM cours;"#;
        
        match DB.db.query(query).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<CoursRow>> = res.take(0) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}