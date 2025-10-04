use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::{RecordId, value::Datetime}
};



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct AnneeAcademiqueRow {
    pub id: RecordId,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct AnneeAcademique {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String
}