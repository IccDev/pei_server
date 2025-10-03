
use common_crates::{
    serde::{self, Deserialize, Serialize},
    surrealdb::RecordId,
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearch {
    pub id: RecordId,
    pub nom: String,
    pub photo: Option<String>,
    pub prenom: String,
    pub langues: Vec<String>,
    pub professions: Vec<Profession>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Profession {
    domaine: Option<String>,
    titre: Option<String>,
    periode_debut: Option<String>,
    periode_fin: Option<String>,
    task: Option<String>
}


#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "self::serde")]
pub struct RegisterFormData {
  personnel: PersonnelData,
  eglise: EgliseData,
  professionnel: ProfessionnelData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct RegisterFormDataResult {
  id: RecordId,
  personnel: PersonnelData,
  eglise: EgliseData,
  professionnel: ProfessionnelData,
}


#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "self::serde")]
pub struct PersonnelData {
    nom: Option<String>,
    prenom: Option<String>,
    genre: Option<String>,
    email: Option<String>,
    consentement_email: Option<bool>,
    photo: Option<String>,
    gsm: Option<String>,
    consentement_gsm: Option<bool>,
    residence: Option<Residence>,
    langues: Vec<String>
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Residence {
    pays: Option<String>,
    ville: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "self::serde")]
pub struct EgliseData {
    eglise: Option<String>,
    star: Option<bool>,
    departements: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "self::serde")]
pub struct ProfessionnelData {
    educations: Vec<Education>,
    professions: Vec<Profession>,
    diplomes: Vec<Diplome>,
    certifications: Vec<Certification>,
    competences: Vec<Competence>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Education {
    pub domaine: Option<String>,
    pub titre: Option<String>,
    pub specialite: Option<String>,
    pub periode_debut: Option<String>,
    pub periode_fin: Option<String>,
    pub competences_acquises: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Diplome {
    nom: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Certification {
    nom: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Competence {
    nom: Option<String>
}