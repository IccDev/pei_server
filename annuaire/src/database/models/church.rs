use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};
use crate::database::models::Adresse;



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Eglise {
    pub id: RecordId,
    pub nom: String,
    pub adresse: Adresse,
    pub description: Option<String>
}