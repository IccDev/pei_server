use crate::{
    services::{
        database::{DatabaseService, AnnuaireTableActor}
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait,
    sqlx,
    rayon::prelude::*
};
use inter_services_messages::{annuaire::{User, AnnuaireSearchInput, AnnuaireSearch, AnnuaireSearchOutput, AnnuaireSearchResponse}, ResponseData};
use std::collections::{BTreeMap, HashSet};

#[async_trait::async_trait] 
impl Serve<AnnuaireSearchInput> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: AnnuaireSearchInput, system: &ServiceAssistant<Self>) -> Self::Response {
        self.search_stars(message, &system).await
    }
}

impl DatabaseService {
    pub(crate) async fn search_stars(&self, msg: AnnuaireSearchInput, system: &ServiceAssistant<Self>) -> Result<ResponseData, String> {
        //println!("msg: {msg:#?}");

        let tables = self.get_related_tables(&msg, &system).await;
        let users = self.get_user_ids(&tables, &system).await;
        //println!("users: {users:?}");
        let filter_campus = self.get_campus_from_tables(&tables);
        let users_ids = self.unique_ids(&users);
        let data = self.get_users(&users_ids, &filter_campus).await;

        //println!("users: {users:#?}");

        Ok(ResponseData::Annuaire(AnnuaireSearchOutput {data}))
    }

    fn unique_ids(&self, users: &[AnnuaireSearchResponse]) -> Vec<i32> {
        let users_ids: Vec<i32> = users.iter().map(|u| {
            match u {
                AnnuaireSearchResponse::UserIds(values) => {
                    values.iter().map(|r| {
                        match r.id {
                            Some(d) => d,
                            None => 0
                        }
                    })
                    .filter(|i| *i != 0)
                    .collect()
                },
                _ => vec![]
            }
        })
        .flatten()
        .collect();

        let mut users_ids_set = HashSet::new();
        users_ids.iter().for_each(|e| {
            let _ = users_ids_set.insert(e.to_owned());
        });

        Vec::from_iter(users_ids_set)
    }

    fn get_campus_from_tables(&self, tables: &[AnnuaireSearchResponse]) -> Vec<i32> {
        let mut result = vec![];

        for t in tables {
            match t {
                AnnuaireSearchResponse::ByKeyResponse((t_name, row_ids)) => {
                    let ids: Vec<i32> = row_ids.iter().map(|i| i.id.unwrap_or_default()).collect();
                    match t_name.as_ref() {
                        "campus" => {
                            result.extend_from_slice(&ids);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        };

        result
    }

    async fn get_user_ids(&self, tables: &[AnnuaireSearchResponse], system: &ServiceAssistant<Self>) -> Vec<AnnuaireSearchResponse> {
        let mut result = vec![];

        for t in tables {
            match t {
                AnnuaireSearchResponse::ByKeyResponse((t_name, row_ids)) => {
                    let ids: Vec<i32> = row_ids.iter().map(|i| i.id.unwrap_or_default()).collect();
                    match t_name.as_ref() {
                        "competences" if ids.len() > 0 => {
                            if let Ok(Ok(user_competences)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_competences".to_string(), 
                                AnnuaireSearch::ByIds(("id_competence".to_string(), ids.clone()))
                            ).await {
                                result.push(user_competences);
                            }
                        },
                        "diplomes_certificats" if ids.len() > 0 => {
                            if let Ok(Ok(user_competences)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_diplomes".to_string(), 
                                AnnuaireSearch::ByIds(("id_diplome".to_string(), ids.clone()))
                            ).await {
                                result.push(user_competences);
                            }
                        },
                        "entreprises" if ids.len() > 0 => {
                            if let Ok(Ok(user_entreprises)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_entreprises".to_string(), 
                                AnnuaireSearch::ByIds(("id_entreprise".to_string(), ids.clone()))
                            ).await {
                                result.push(user_entreprises);
                            }
                        },
                        "ecoles" if ids.len() > 0 => {
                            if let Ok(Ok(user_ecoles)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_ecoles".to_string(), 
                                AnnuaireSearch::ByIds(("id_ecole".to_string(), ids.clone()))
                            ).await {
                                result.push(user_ecoles);
                            }
                        },
                        "titres" if ids.len() > 0 => {
                            if let Ok(Ok(user_titres)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_titres".to_string(), 
                                AnnuaireSearch::ByIds(("id_titre".to_string(), ids.clone()))
                            ).await {
                                result.push(user_titres);
                            }
                        },
                        "specialites" if ids.len() > 0 => {
                            if let Ok(Ok(user_specialites)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_specialites".to_string(), 
                                AnnuaireSearch::ByIds(("id_specialite".to_string(), ids.clone()))
                            ).await {
                                result.push(user_specialites);
                            }
                        },
                        "domaines" if ids.len() > 0 => {
                            if let Ok(Ok(user_domaines)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_domaines".to_string(), 
                                AnnuaireSearch::ByIds(("id_domaine".to_string(), ids.clone()))
                            ).await {
                                result.push(user_domaines);
                            }
                        },
                        "departements" if ids.len() > 0 => {
                            if let Ok(Ok(user_department)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_department".to_string(), 
                                AnnuaireSearch::ByIds(("id_departement".to_string(), ids.clone()))
                            ).await {
                                result.push(user_department);
                            }
                        },
                        "langues" if ids.len() > 0 => {
                            if let Ok(Ok(user_langues)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_langues".to_string(), 
                                AnnuaireSearch::ByIds(("id_langues".to_string(), ids.clone()))
                            ).await {
                                result.push(user_langues);
                            }
                        },
                        /*"campus" if ids.len() > 0 => {
                            if let Ok(Ok(user_campus)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_campus".to_string(), 
                                AnnuaireSearch::ByIds(("id_campus".to_string(), ids.clone()))
                            ).await {
                                result.push(user_campus);
                            }
                        },*/
                        "localites" if ids.len() > 0 => {
                            if let Ok(Ok(user_localites)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
                                "user_localites".to_string(), 
                                AnnuaireSearch::ByIds(("id_localite".to_string(), ids.clone()))
                            ).await {
                                result.push(user_localites);
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        };

        result
    }

    async fn get_related_tables(&self, msg: &AnnuaireSearchInput, system: &ServiceAssistant<Self>) -> Vec<AnnuaireSearchResponse> {
        let mut result: Vec<_> = vec![];
        
        if let Ok(Ok(icc_user_plus_infos)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "user_plus_infos".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_user_plus_infos);
        }

        if let Ok(Ok(icc_competences)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "competences".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_competences);
        }

        if let Ok(Ok(icc_diplomes_certificats)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "diplomes_certificats".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_diplomes_certificats);
        }

        if let Ok(Ok(icc_entreprises)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "entreprises".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_entreprises);
        }

        if let Ok(Ok(icc_ecoles)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "ecoles".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_ecoles);
        }

        if let Ok(Ok(icc_titres)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "titres".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_titres);
        }

        if let Ok(Ok(icc_specialites)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "specialites".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_specialites);
        }

        if let Ok(Ok(icc_domaines)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "domaines".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_domaines);
        }

        if let Ok(Ok(icc_departements)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "departements".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_departements);
        }

        if let Ok(Ok(icc_langues)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "langues".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_langues);
        }

        if let Ok(Ok(icc_campus)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "campus".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_campus);
        }

        if let Ok(Ok(icc_localites)) = system.call_actor::<AnnuaireTableActor, AnnuaireSearch>(
            "localites".to_string(), 
            AnnuaireSearch::ByKey(msg.clone())
        ).await {
            result.push(icc_localites);
        }

        result
    }

    async fn get_users(&self, ids: &[i32], filter_campus: &[i32]) -> Vec<User> {
        let mut users = self.users_by_ids(&ids, &filter_campus).await;
        //println!("users: {users:?}");
        for user in &mut users {
            user.departements = self.departements_by_user_id(&user.id.unwrap_or_default()).await;
            user.competences = self.competence_by_user_id(&user.id.unwrap_or_default()).await;
            user.campus = self.campus_by_user_id(&user.id.unwrap_or_default()).await;
            user.contact = self.contact_by_user_id(&user.id.unwrap_or_default()).await;
            user.diplomes = self.diplomes_by_user_id(&user.id.unwrap_or_default()).await;
            user.domaines = self.domaines_by_user_id(&user.id.unwrap_or_default()).await;
            user.ecoles = self.ecoles_by_user_id(&user.id.unwrap_or_default()).await;
            user.entreprises = self.entreprises_by_user_id(&user.id.unwrap_or_default()).await;
            user.langues = self.langues_by_user_id(&user.id.unwrap_or_default()).await;
            user.localites = self.localites_by_user_id(&user.id.unwrap_or_default()).await;
            user.specialites = self.specialites_by_user_id(&user.id.unwrap_or_default()).await;
            user.titres = self.titres_by_user_id(&user.id.unwrap_or_default()).await;
            //user.user_plus_infos = self.user_plus_infos_by_user_id(&user.id.unwrap_or_default()).await;
        }

        users
    }
}