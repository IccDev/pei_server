use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::departements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Departement {
    pub id: i32,
    pub nom: String,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::departements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DepartementInsert {
    pub nom: String,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct DepartementNew {
    pub nom: String,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct DepartementData {
    pub id: i32,
    pub nom: String,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&Departement> for DepartementData {
    fn from(value: &Departement) -> DepartementData {
        DepartementData {
            id: value.id.clone(),
            nom: value.nom.clone(),
            abbreviation: value.abbreviation.clone(),
            description: value.description.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}