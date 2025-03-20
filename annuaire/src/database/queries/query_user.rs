use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result
};
use crate::{
    DB,
    router::{BoxedBody, ok, err},
    DBService,
    database::models::RegisterFormDataResult
};



impl DBService {
    pub async fn query_user(&self, id: String) -> Response<BoxedBody> {
        let query = user_query(&id);
        
        match DB.db.query(&query).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<RegisterFormDataResult>> = res.take(0) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}




fn user_query(id: &str) -> String {
    format!("select * from users where id = users:⟨{id}⟩;")
}
