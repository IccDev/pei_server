use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};
use super::Cours;


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct DisciplineRow {
    pub id: RecordId,
    pub name: String,
    pub description: String,
    pub cours: Vec<RecordId>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Discipline {
    pub name: String,
    pub description: String,
    pub cours: Vec<RecordId>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct DisciplineData {
    pub id: RecordId,
    pub name: String,
    pub description: String,
    pub cours: Vec<Cours>
}