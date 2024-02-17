mod model;

pub use model::*;
use common::{
    acteur::Acteur, 
    sqlx::{FromRow, Row, Error, postgres::PgRow},
    serde::{self, Deserialize, Serialize}
};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireMessage {
    Search(AnnuaireSearchInput)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearchInput {
    pub key: Option<String>,
    pub church: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearchOutput {
    pub data: Vec<User>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(crate = "self::serde")]
pub struct RowId {
    pub id: Option<i32>
}

impl FromRow<'_, PgRow> for RowId {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?)
        })
    }
}

impl Hash for RowId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}