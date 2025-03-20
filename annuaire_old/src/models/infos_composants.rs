use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::infos_composants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InfosComposant {
    pub id: i32,
    pub composant_id: i32,
    pub parcours_id: i32,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::infos_composants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InfosComposantInsert {
    pub composant_id: i32,
    pub parcours_id: i32,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct InfosComposantNew {
    pub id: i32,
    pub composant_id: i32,
    pub parcours_id: i32,
    pub description: Option<String>,
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct InfosComposantData {
    pub id: i32,
    pub nom: Option<String>,
    pub composant_id: i32,
    pub parcours_id: i32,
    pub composant: Option<String>,
    pub parcours: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&InfosComposant> for InfosComposantData {
    fn from(value: &InfosComposant) -> InfosComposantData {
        InfosComposantData {
            id: value.id.clone(),
            nom: None,
            parcours_id: value.parcours_id.clone(),
            composant_id: value.composant_id.clone(),
            composant: None,
            parcours: None,
            description: value.description.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}