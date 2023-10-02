use serde_derive::{Deserialize, Serialize};
use icc_common::sqlx::{FromRow, Row, Error, postgres::PgRow};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnuaireSearch {
    pub key: Option<String>,
    pub church: Option<String>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnuaireSearchResponse {
    pub stars: Vec<Star>
}


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


/*use icc_common::{
    time::OffsetDateTime,
    uuid::Uuid,
    sqlx::{FromRow, Row, postgres::PgRow, Result, Error}
};*/
/*
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub activated: bool,
    pub email: String,
    pub last_name: String,
    pub first_name: String,
    pub password: String,
    pub is_admin: bool,
    pub totp: String,
    pub two_factor: bool,
    pub created_at: Option<OffsetDateTime>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub last_name: String,
    pub first_name: String,
    pub password: String,
    pub user_token: String,
    pub email: String,
    pub two_factor: bool
}
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginForm(pub String);

/*
impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self> {
        let id = row.try_get("id")?;
        let Ok(id_uuid) = Uuid::parse_str(id) else {
            return Err(Error::TypeNotFound {type_name: String::from("uuid")});
        };

        Ok(Self {
            id: id_uuid,
            activated: row.try_get("activated")?,
            email: row.try_get("email")?,
            last_name: row.try_get("last_name")?,
            first_name: row.try_get("first_name")?,
            password: "**********".to_string(),
            is_admin: row.try_get("is_admin")?,
            totp: "**********".to_string(),
            two_factor: row.try_get("two_factor")?,
            created_at: Some(row.try_get("created_at")?)
        })
    }
}
*/