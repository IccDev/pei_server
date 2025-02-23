use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct CountryRow {
    pub id: RecordId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Country {
    pub name: String,
}