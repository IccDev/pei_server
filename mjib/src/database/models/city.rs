use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};
use crate::database::models::CountryRow;



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct CityRow {
    pub id: RecordId,
    pub country_id: RecordId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct City {
    pub country_id: RecordId,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct CityData {
    pub id: RecordId,
    pub country: CountryRow,
    pub name: String,
}