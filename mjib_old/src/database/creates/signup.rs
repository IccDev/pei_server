use common_crates::{
    hyper::Response,
    serde_json::json,
    surrealdb::opt::auth::Record
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{User, UserRow}
};
use crate::addresses::{mjib_access_method, mjib_db_db, mjib_db_ns};


impl DBService {
    pub async fn signup(&self, user: User) -> Response<BoxedBody> {
        match  DB.db.signup(Record {
            namespace: &mjib_db_ns(),
            database: &mjib_db_db(),
            access: &mjib_access_method(),
            params: user,
        }).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}