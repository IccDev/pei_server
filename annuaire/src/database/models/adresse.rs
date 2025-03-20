use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId,
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Adresse {
    pub boite: Option<i32>,
    pub code_postal: Option<String>,
    pub commune: Option<String>,
    pub numero: Option<i32>,
    pub pays: String,
    pub rue: Option<String>,
    pub ville: String
}