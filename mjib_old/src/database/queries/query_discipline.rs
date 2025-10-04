use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::DisciplineData
};



impl DBService {
    pub async fn query_disciplines(&self) -> Response<BoxedBody> {
        let query = r#"
            SELECT id, 
            (SELECT * FROM cours WHERE id in $parent.cours) 
            AS cours,
            name,
            description
            FROM discipline;
        "#;
        
        match DB.db.query(query).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<DisciplineData>> = res.take(0) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}