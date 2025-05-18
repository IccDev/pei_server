use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::{RecordId, value::Datetime},
    chrono::offset::Utc
};
use crate::database::models::BrancheData;



#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserBranchesRow {
    pub id: RecordId,
    pub user_id: RecordId,
    pub branches: Vec<RecordId>
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserBranches {
    pub user_id: RecordId,
    pub branches: Vec<RecordId>
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserBranchesData {
    pub id: RecordId,
    pub user_id: RecordId,
    pub branches: Vec<BrancheData>
}