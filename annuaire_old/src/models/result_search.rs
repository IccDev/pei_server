use common_crates::serde::{self, Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct SearchResult {
    pub nom: String,
    pub prenom: String,
    pub photo: String,
    pub eglise: String,
    pub professions: Vec<String>
}

