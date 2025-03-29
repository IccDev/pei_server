use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct CoursRow {
    pub id: RecordId,
    pub name: String,
    pub location: String,
    pub description: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Cours {
    pub name: String,
    pub location: String,
    pub description: String
}