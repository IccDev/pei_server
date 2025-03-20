use common_crates::serde::{self, Deserialize, Serialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "self::serde")]
pub struct InfosToCreateUser {
    pub eglises: Vec<EgliseData>,
    pub langues: Vec<LangueData>,
    pub departements: Vec<DepartementData>,
    pub eglise_departements: Vec<EgliseDepartementData>,
    pub composants: Vec<ComposantData>,
    pub qualifications: Vec<QualificationData>,
    pub infos_qualifications: Vec<InfosQualificationData>,
    pub infos_composants: Vec<InfosComposantData>,
    pub parcours: Vec<ParcoursData>,
}

/*
// qualifications
certificats
competences
diplomes
*/

/*
// composants
domaines
ecoles
entreprises
specialites
titres
*/