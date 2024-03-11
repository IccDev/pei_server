use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait
};
use inter_services_messages::{annuaire::{User, RowId, AnnuaireResponse/*, AnnuaireSearchResponse*/}, ResponseData};
use std::collections::{BTreeMap, HashSet};

#[async_trait::async_trait] 
impl Serve<User> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: User, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.create_star(message).await
    }
}

impl DatabaseService {
    
    pub async fn create_star(&self, msg: User) -> Result<ResponseData, String> {
        
        
        /*
        
        {
                "user_plus_infos": {
                    "description": "Je cherche Dieu!"
                }
            }


        let tables = self.get_related_tables(&msg).await;
        let users = self.get_user_ids(&tables).await;
        let data = self.get_users(&users, msg.church.as_deref()).await;
        
        pub struct User {
        pub diplomes: Vec<DiplomeCertificat>,
        pub competences: Vec<Competence>,
        pub user_plus_infos: Option<UserPlusInfos>
    }

        */


        let user_id = self.save_user(&msg).await;
        println!("user_id: {user_id:#?}");
        // Campus
        let campus_id = msg.campus.unwrap_or_default().id.unwrap_or_default();
        let _ = self.create_user_campus(&user_id, &campus_id).await;
        
        // Departement
        for msg_departement in &msg.departements {
            let _ = self.create_user_department(&user_id, &campus_id, &msg_departement.id.unwrap_or_default()).await;
        }
        // Contact
        let _ = self.save_contact(user_id, &msg.contact.unwrap_or_default()).await;
        // Langues
        for msg_langue in &msg.langues {
            let _ = self.create_user_langue(&user_id, &msg_langue.id.unwrap_or_default()).await;
        }
        // Domaines
        for msg_domaine in &msg.domaines {
            let domain_id = self.create_domaine(&msg_domaine).await;
            let _ = self.create_user_domaine(&user_id, &domain_id).await;
        }
        // Titres
        /*
        let mut titre_ids = vec![];
        for msg_titre in &msg.titres {
            let _ = self.save_titre(&msg_titre).await;
        }
        */
        // Specialite
        for msg_specialite in &msg.specialites {
            let specialite_id = self.create_specialite(&msg_specialite).await;
            let _ = self.create_user_specialite(&user_id, &specialite_id).await;
        }
        // Ecole
        
        for msg_ecole in &msg.ecoles {
            let ecole_locale = &msg_ecole.clone().localite.unwrap_or_default();
            let ecole_id = self.save_ecole(&msg_ecole, &ecole_locale).await;
            let _ = self.create_user_ecole(&user_id, &ecole_id, &ecole_locale.clone().consentement.unwrap_or_default()).await;
        }
        // Entreprise
        for msg_entreprise in &msg.entreprises {
            let locale = &msg_entreprise.clone().localite.unwrap_or_default();
            let entreprise_locale_id = self.create_localite(&locale).await;
            let entr_id = self.create_entreprise(entreprise_locale_id, &msg_entreprise).await;
            let _ = self.create_user_entreprise(&user_id, &entr_id, &locale.consentement.unwrap_or_default()).await;
        }
        // Localite
        let user_locale_id = self.create_localite(&msg.localites.unwrap_or_default()).await;
        // DiplomeCertificat
        for msg_diplome in &msg.diplomes {
            let diplome_id = self.create_diplome(&msg_diplome).await;
            let _ = self.create_user_diplome(&user_id, &diplome_id, &false).await;
        }

        // Competences
        for msg_competence in &msg.competences {
            let competence_id = self.create_competence(&msg_competence).await;
            let _ = self.create_user_competences(&user_id, &competence_id, false).await;
        }

        // user_plus_infos
        let _ = self.create_user_plus_infos(&user_id, &msg.user_plus_infos.unwrap_or_default()).await;

        Ok(ResponseData::Annuaire(AnnuaireResponse::Create(user_id)))
    }
}