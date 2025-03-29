use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};
use crate::database::models::{AnneeAcademique, DisciplineData};


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct BrancheRow {
    pub id: RecordId,
    pub name: String,
    pub annee_academique: RecordId,
    pub description: String,
    pub disciplines: Vec<RecordId>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Branche {
    pub name: String,
    pub annee_academique: RecordId,
    pub description: String,
    pub disciplines: Vec<RecordId>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct BrancheData {
    pub id: RecordId,
    pub name: String,
    pub annee_academique: AnneeAcademique,
    pub description: String,
    pub disciplines: Vec<DisciplineData>
}