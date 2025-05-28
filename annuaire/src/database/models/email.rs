use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId
};
use crate::database::models::Adresse;



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserId {
    pub id: RecordId
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Email {
    pub email: String
}