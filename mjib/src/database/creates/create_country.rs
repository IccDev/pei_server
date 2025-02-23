use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{Country, CountryRow}
};



impl DBService {
    pub async fn create_countries(&self, countries: Vec<Country>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<CountryRow>>("country").content(countries).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}