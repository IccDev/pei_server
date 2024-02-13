mod model;

pub use model::*;
use serde_derive::{Deserialize, Serialize};
use icc_common::{ acteur::Acteur, sqlx::{FromRow, Row, Error, postgres::PgRow}};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnuaireSearchInput {
    pub key: Option<String>,
    pub church: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnnuaireSearch {
    ByKey(AnnuaireSearchInput),
    ByIds((String, Vec<i32>))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnnuaireSearchResponse {
    ByKeyResponse((String, Vec<RowId>)),
    UserIds(Vec<RowId>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnuaireSearchOutput {
    pub data: Vec<User>
}

/*
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Star {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub departements: Vec<Meta>,
    pub metiers: Vec<Meta>,
    pub eglises: Vec<Meta>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawStar {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub departement: Option<String>,
    pub departement_desc: Option<String>,
    pub metier: Option<String>,
    pub metier_desc: Option<String>,
    pub eglise: Option<String>,
    pub eglise_desc: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Meta {
    pub nom: Option<String>,
    pub desc: Option<String>
}


impl FromRow<'_, PgRow> for RawStar {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            nom: Some(row.try_get("nom")?),
            prenom: Some(row.try_get("prenom")?),
            email: Some(row.try_get("email")?),
            telephone: Some(row.try_get("telephone")?),
            departement: Some(row.try_get("departement")?),
            departement_desc: Some(row.try_get("departement_desc")?),
            metier: Some(row.try_get("metier")?),
            metier_desc: Some(row.try_get("metier_desc")?),
            eglise: Some(row.try_get("eglise")?),
            eglise_desc: Some(row.try_get("eglise_desc")?)
        })
    }
}
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
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