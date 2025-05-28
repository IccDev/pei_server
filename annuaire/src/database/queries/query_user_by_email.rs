use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result
};
use crate::{
    DB,
    router::{BoxedBody, ok, err},
    DBService,
    database::models::UserId
};



impl DBService {
    pub async fn query_user_by_email(&self, email: String) -> Response<BoxedBody> {
        let query = user_query(&email);
        match DB.db.query(&query).await {
            Ok(mut res) => {
                let Ok(value): Result<Vec<UserId>> = res.take(0) else {
                    return ok(BoxedBody::new(json!({"error": "not working"}).to_string()));
                };
                ok(BoxedBody::new(json!({"data": value}).to_string()))
            },
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}


fn user_query(email: &str) -> String {
    format!("select id from users where personnel.email = '{email}';")
}
