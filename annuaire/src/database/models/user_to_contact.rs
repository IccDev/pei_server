use common_crates::serde::{self, Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct UserToContact {
    pub nom: String,
    pub prenom: String,
    pub email: String
}