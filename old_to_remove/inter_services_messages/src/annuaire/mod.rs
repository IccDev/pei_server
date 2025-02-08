mod model;

pub use model::*;
use common::{
    sqlx::{FromRow, Row, Error, postgres::PgRow},
    serde::{self, Deserialize, Serialize}
};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireMessage {
    Search(AnnuaireSearchInput),
    CreateUser(User),
    CreateCampus(Campus),
    CreateCompetences(Competence),
    CreateDepartement(Departement),
    CreateDiplomes(DiplomeCertificat),
    CreateDomaine(Domaine),
    CreateEcole(Ecole),
    CreateEntreprise(Entreprise),
    CreateLangue(Langue),
    CreateLocalite(Localite),
    CreateSpecialite(Specialite),
    CreateTitre(Titre),
    CreateUserInfo(UserPlusInfos),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireResponse {
    Search(AnnuaireSearchOutput),
    Create(i32)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearchInput {
    pub key: Option<String>,
    pub church: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearchOutput {
    pub data: Vec<User>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(crate = "self::serde")]
pub struct RowId {
    pub id: Option<i32>
}

impl FromRow<'_, PgRow> for RowId {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?)
        })
    }
}

impl Hash for RowId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
