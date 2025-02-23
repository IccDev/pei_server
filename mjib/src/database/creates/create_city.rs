use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{City, CityRow}
};



impl DBService {
    pub async fn create_cities(&self, cities: Vec<City>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<CityRow>>("city").content(cities).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}