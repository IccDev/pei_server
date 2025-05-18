use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::{RecordId, opt::Resource}
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{RegisterFormData, RegisterFormDataResult}
};

impl DBService {
    pub async fn update_user(&self, user_id: &str, user: RegisterFormData) -> Response<BoxedBody> {
        match DB.db.update(Resource::RecordId(RecordId::from_table_key("users", user_id))).content(user).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}