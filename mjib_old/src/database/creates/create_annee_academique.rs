use common_crates::{
    hyper::Response,
    serde_json::json,
};
use crate::{
    DB,
    router::{BoxedBody, ok},
    DBService,
    database::models::{AnneeAcademique, AnneeAcademiqueRow}
};



impl DBService {
    pub async fn create_annee_academique(&self, annee_academique: Vec<AnneeAcademique>) -> Response<BoxedBody> {
        match DB.db.insert::<Vec<AnneeAcademiqueRow>>("annee_academique").content(annee_academique).await {
            Ok(res) => ok(BoxedBody::new(json!({"data": res}).to_string())),
            Err(e) => ok(BoxedBody::new(json!({"error": e}).to_string()))
        }
    }
}