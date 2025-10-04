use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::Result,
    surrealdb::opt::auth::Record,
    tracing::info
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{UserData, AuthParams}
};
use crate::addresses::{mjib_access_method, mjib_db_db, mjib_db_ns};

impl DBService {
    pub async fn signin(&self, auth_params: AuthParams) -> Response<BoxedBody> {
        match DB.db.signin(Record {
            namespace: &mjib_db_ns(),
            database: &mjib_db_db(),
            access: &mjib_access_method(),
            params: auth_params,
        }).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}