use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::eglise_departements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EgliseDepartement {
    pub id: i32,
    pub eglise_id: i32,
    pub departement_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::eglise_departements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EgliseDepartementInsert {
    pub eglise_id: i32,
    pub departement_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct EgliseDepartementNew {
    pub eglise_id: i32,
    pub departement_id: i32,
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct EgliseDepartementData {
    pub id: i32,
    pub eglise_id: i32,
    pub departement_id: i32,
    pub departement: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&EgliseDepartement> for EgliseDepartementData {
    fn from(value: &EgliseDepartement) -> EgliseDepartementData {
        EgliseDepartementData {
            id: value.id.clone(),
            eglise_id: value.eglise_id.clone(),
            departement_id: value.departement_id.clone(),
            departement: None,
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}