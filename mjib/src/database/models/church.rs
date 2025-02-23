use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};
use crate::database::models::CityData;



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ChurchRow {
    pub id: RecordId,
    pub city_id: RecordId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Church {
    pub city_id: RecordId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ChurchData {
    pub id: RecordId,
    pub city: CityData,
    pub name: String,
}