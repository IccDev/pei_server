use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::parcours)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Parcours {
    pub id: i32,
    pub nom: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::parcours)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ParcoursInsert {
    pub id: i32,
    pub nom: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ParcoursNew {
    pub nom: String,
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ParcoursData {
    pub id: i32,
    pub nom: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&Parcours> for ParcoursData {
    fn from(value: &Parcours) -> ParcoursData {
        ParcoursData {
            id: value.id.clone(),
            nom: value.nom.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}