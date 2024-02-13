use serde_derive::{Deserialize, Serialize};
use icc_common::sqlx::{FromRow, Row, Error, postgres::PgRow};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: Option<i32>,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub consentement_nom: Option<bool>,
    pub photo: Option<String>,
    pub campus: Vec<Campus>,
    pub departements: Vec<Departement>,
    pub contact: Option<Contact>,
    pub langues: Vec<Langue>,
    pub domaines: Vec<Domaine>,
    pub titres: Vec<Titre>,
    pub specialites: Vec<Specialite>,
    pub ecoles: Vec<Ecole>,
    pub entreprises: Vec<Entreprise>,
    pub localites: Vec<Localite>,
    pub diplomes: Vec<DiplomeCertificat>,
    pub competences: Vec<Competence>,
    pub user_plus_infos: Option<UserPlusInfos>
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            prenom: Some(row.try_get("prenom")?),
            consentement_nom: Some(row.try_get("consentement_nom")?),
            photo: Some(row.try_get("photo")?),
            ..User::default()
        })
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Localite {
    pub id: Option<i32>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub code_postal: Option<String>,
    pub commune: Option<String>,
    pub quartier: Option<String>,
    pub adresse: Option<String>,
    pub consentement: Option<bool>,
}
impl FromRow<'_, PgRow> for Localite {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            pays: Some(row.try_get("pays")?),
            ville: Some(row.try_get("ville")?),
            code_postal: Some(row.try_get("code_postal")?),
            commune: Some(row.try_get("commune")?),
            quartier: Some(row.try_get("quartier")?),
            adresse: Some(row.try_get("adresse")?),
            consentement: Some(row.try_get("consentement")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Campus {
    pub id: Option<i32>,
    pub localite: Option<Localite>,
    pub nom: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for Campus {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?),
            localite: None
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Langue {
    pub id: Option<i32>,
    pub nom: Option<String>,
    pub abbreviation: Option<String>
}

impl FromRow<'_, PgRow> for Langue {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            abbreviation: Some(row.try_get("abbreviation")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Departement {
    pub id: Option<i32>,
    pub campus: Option<Campus>,
    pub nom: Option<String>,
    pub abbreviation: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for Departement {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            //campus: Some(row.try_get("nom")?),
            nom: Some(row.try_get("nom")?),
            abbreviation: Some(row.try_get("abbreviation")?),
            description: Some(row.try_get("description")?),
            ..Departement::default()
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Domaine {
    pub id: Option<i32>,
    pub nom: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for Domaine {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Titre {
    pub id: Option<i32>,
    pub nom: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for Titre {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Specialite {
    pub id: Option<i32>,
    pub nom: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for Specialite {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Ecole {
    pub id: Option<i32>,
    pub localite: Option<Localite>,
    pub nom: Option<String>,
    pub description: Option<String>,
    pub consentement: Option<bool>
}

impl FromRow<'_, PgRow> for Ecole {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?),
            consentement: Some(row.try_get("consentement")?),
            localite: None
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Entreprise {
    pub id: Option<i32>,
    pub localite: Option<Localite>,
    pub nom: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for Entreprise {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?),
            localite: None
        })
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DiplomeCertificat {
    pub id: Option<i32>,
    pub nom: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for DiplomeCertificat {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Competence {
    pub id: Option<i32>,
    pub nom: Option<String>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for Competence {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            nom: Some(row.try_get("nom")?),
            description: Some(row.try_get("description")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserPlusInfos {
    pub id: Option<i32>,
    pub description: Option<String>
}

impl FromRow<'_, PgRow> for UserPlusInfos {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            description: Some(row.try_get("description")?)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Contact {
    pub id: Option<i32>,
    pub gsm: Option<String>,
    pub email: Option<String>,
    pub consentement_gsm: Option<bool>,
    pub consentement_email: Option<bool>
}

impl FromRow<'_, PgRow> for Contact {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: Some(row.try_get("id")?),
            gsm: Some(row.try_get("gsm")?),
            email: Some(row.try_get("email")?),
            consentement_gsm: Some(row.try_get("consentement_gsm")?),
            consentement_email: Some(row.try_get("consentement_email")?)
        })
    }
}