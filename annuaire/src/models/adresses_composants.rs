use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::adresses_composants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AdresseComposant {
    pub id: i32,
    pub composant_id: i32,
    pub adresse_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}
/*
#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::adresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AdresseInsert {
    pub pays: String,
    pub ville: String,
    pub commune: Option<String>,
    pub code_postal: Option<String>,
    pub rue: Option<String>,
    pub numero: Option<i32>,
    pub boite: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct AdresseNew {
    pub pays: String,
    pub ville: String,
    pub commune: Option<String>,
    pub code_postal: Option<String>,
    pub rue: Option<String>,
    pub numero: Option<i32>,
    pub boite: Option<i32>,
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct AdresseData {
    pub id: i32,
    pub pays: String,
    pub ville: String,
    pub commune: Option<String>,
    pub code_postal: Option<String>,
    pub rue: Option<String>,
    pub numero: Option<i32>,
    pub boite: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&Adresse> for AdresseData {
    fn from(value: &Adresse) -> AdresseData {
        AdresseData {
            id: value.id.clone(),
            pays: value.pays.clone(),
            ville: value.ville.clone(),
            commune: value.commune.clone(),
            code_postal: value.code_postal.clone(),
            rue: value.rue.clone(),
            numero: value.numero.clone(),
            boite: value.boite.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}
*/