use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::consentements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Consentement {
    pub id: i32,
    pub nom: bool,
    pub gsm: bool,
    pub email: bool,
    pub ecole: bool,
    pub diplome: bool,
    pub certificat: bool,
    pub entreprise: bool,
    pub adresse: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::consentements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConsentementInsert {
    pub nom: bool,
    pub gsm: bool,
    pub email: bool,
    pub ecole: bool,
    pub diplome: bool,
    pub certificat: bool,
    pub entreprise: bool,
    pub adresse: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ConsentementNew {
    pub nom: bool,
    pub gsm: bool,
    pub email: bool,
    pub ecole: bool,
    pub diplome: bool,
    pub certificat: bool,
    pub entreprise: bool,
    pub adresse: bool,
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ConsentementData {
    pub id: i32,
    pub nom: bool,
    pub gsm: bool,
    pub email: bool,
    pub ecole: bool,
    pub diplome: bool,
    pub certificat: bool,
    pub entreprise: bool,
    pub adresse: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&Consentement> for ConsentementData {
    fn from(value: &Consentement) -> ConsentementData {
        ConsentementData {
            id: value.id.clone(),
            nom: value.nom.clone(),
            gsm: value.gsm.clone(),
            email: value.email.clone(),
            ecole: value.ecole.clone(),
            diplome: value.diplome.clone(),
            certificat: value.certificat.clone(),
            entreprise: value.entreprise.clone(),
            adresse: value.adresse.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}