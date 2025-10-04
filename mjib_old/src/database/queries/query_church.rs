use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::ChurchData
};



impl DBService {
    pub async fn query_churches(&self) -> Response<BoxedBody> {
        let query = r#"
            LET $city_records = (SELECT id, 
                (SELECT * FROM ONLY country WHERE id = $parent.country_id LIMIT 1) 
                AS country,
                name
                FROM city);

            RETURN SELECT id, 
                (SELECT * FROM ONLY $city_records WHERE id = $parent.city_id LIMIT 1) 
                AS city,
                name
                FROM church;
        "#;
        
        match DB.db.query(query).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<ChurchData>> = res.take(1) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}