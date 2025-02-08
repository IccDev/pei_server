use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub id: uuid::Uuid,
    pub consentement_id: i32,
    pub eglise_id: i32,
    pub adresse_id: i32,
    pub nom: String,
    pub prenom: String,
    pub photo: Option<String>,
    pub email: Option<String>,
    pub gsm: Option<String>,
    pub icc_star: bool,
    pub plus_infos: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProfileInsert {
    pub consentement_id: i32,
    pub eglise_id: i32,
    pub adresse_id: i32,
    pub nom: String,
    pub prenom: String,
    pub photo: Option<String>,
    pub email: Option<String>,
    pub gsm: Option<String>,
    pub icc_star: bool,
    pub plus_infos: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ProfileNew {
    pub consentement_id: i32,
    pub eglise_id: i32,
    pub adresse_id: i32,
    pub nom: String,
    pub prenom: String,
    pub photo: Option<String>,
    pub email: Option<String>,
    pub gsm: Option<String>,
    pub icc_star: bool,
    pub plus_infos: Option<String>,
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ProfileData {
    pub id: uuid::Uuid,
    pub consentement_id: i32,
    pub eglise_id: i32,
    pub adresse_id: i32,
    pub nom: String,
    pub prenom: String,
    pub photo: Option<String>,
    pub email: Option<String>,
    pub gsm: Option<String>,
    pub icc_star: bool,
    pub plus_infos: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&Profile> for ProfileData {
    fn from(value: &Profile) -> ProfileData {
        ProfileData {
            id: value.id.clone(),
            consentement_id: value.consentement_id.clone(),
            eglise_id: value.eglise_id.clone(),
            adresse_id: value.adresse_id.clone(),
            nom: value.nom.clone(),
            prenom: value.prenom.clone(),
            photo: value.photo.clone(),
            email: value.email.clone(),
            gsm: value.gsm.clone(),
            icc_star: value.icc_star.clone(),
            plus_infos: value.plus_infos.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}