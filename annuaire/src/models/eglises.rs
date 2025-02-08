use diesel::prelude::*;
use super::{Adresse, AdresseNew, AdresseData};
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::eglises)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Eglise {
    pub id: i32,
    pub nom: String,
    pub adresse_id: i32,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::eglises)]
pub struct EgliseInsert {
    pub nom: String,
    pub adresse_id: i32,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct EgliseNew {
    pub nom: String,
    pub description: Option<String>,
    pub adresse: AdresseNew
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "self::serde")]
pub struct EgliseData {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub adresse: AdresseData,
}

impl From<&Eglise> for EgliseData {
    fn from(value: &Eglise) -> EgliseData {
        EgliseData {
            id: value.id.clone(),
            nom: value.nom.clone(),
            description: value.description.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            adresse: AdresseData::default()
        }
    }
}