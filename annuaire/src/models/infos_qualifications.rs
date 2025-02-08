use diesel::prelude::*;
use common_crates::{
    serde::{self, Deserialize, Serialize},
    chrono::{DateTime, Utc}
};

#[derive(Queryable, Identifiable, Selectable, AsChangeset, Debug, PartialEq, Clone, Default)]
#[diesel(table_name = crate::schema::infos_qualifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InfosQualification {
    pub id: i32,
    pub qualification_id: i32,
    pub profile_id: uuid::Uuid,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::infos_qualifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InfosQualificationInsert {
    pub qualification_id: i32,
    pub profile_id: uuid::Uuid,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct InfosQualificationNew {
    pub qualification_id: i32,
    pub profile_id: uuid::Uuid,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct InfosQualificationData {
    pub id: i32,
    pub qualification_id: i32,
    pub qualification: Option<String>,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>
}

impl From<&InfosQualification> for InfosQualificationData {
    fn from(value: &InfosQualification) -> InfosQualificationData {
        InfosQualificationData {
            id: value.id.clone(),
            qualification_id: value.qualification_id.clone(),
            qualification: None,
            abbreviation: value.abbreviation.clone(),
            description: value.description.clone(),
            created_at: value.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
            updated_at: value.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M"))))
        }
    }
}