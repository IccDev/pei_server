use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{RegisterFormData, RegisterFormDataResult}
};



impl DBService {
    pub async fn create_user(&self, users: RegisterFormData) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<RegisterFormDataResult>>("users").content(users).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}