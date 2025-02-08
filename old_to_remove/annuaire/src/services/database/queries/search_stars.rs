use crate::services::DatabaseService;
use common::{
    acteur::{Serve, ServiceAssistant},
    async_trait,
    sqlx
};
use inter_services_messages::{annuaire::{User, RowId, AnnuaireSearchInput, AnnuaireResponse, AnnuaireSearchOutput/*, AnnuaireSearchResponse*/}, ResponseData};
use std::collections::{BTreeMap, HashSet};

#[async_trait::async_trait] 
impl Serve<AnnuaireSearchInput> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: AnnuaireSearchInput, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.search_stars(message).await
    }
}

impl DatabaseService {
    
    pub async fn search_stars(&self, msg: AnnuaireSearchInput) -> Result<ResponseData, String> {
        let tables = self.get_related_tables(&msg).await;
        let users = self.get_user_ids(&tables).await;
        let data = self.get_users(&users, msg.church.as_deref()).await;

        Ok(ResponseData::Annuaire(AnnuaireResponse::Search( AnnuaireSearchOutput {data})))
    }

    pub fn search_in_table(&self, table: &str, key: &str) -> String {
        format!(r#"SELECT t.id
            FROM annuaire.{table} t
            WHERE tsv @@ to_tsquery('pg_catalog.english', '"{key}"');"#)
    }
    
    async fn get_user_ids(&self, tables: &BTreeMap<String, Vec<RowId>>) -> HashSet<RowId> {
        let mut result = vec![];

        for (t_name, row_ids) in tables {
            let ids: Vec<i32> = row_ids.iter().map(|i| i.id.unwrap_or_default()).collect();
            let ids_vec: Vec<_> = ids.iter().map(|i| format!("{i}")).collect();
            let ids_str = ids_vec.join(",");

            match t_name.as_ref() {
                "competences" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_competences_user_id(&ids_str).await);
                },
                "diplomes_certificats" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_diplomes_user_id(&ids_str).await);
                },
                "entreprises" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_entreprises_user_id(&ids_str).await);
                },
                "ecoles" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_ecoles_user_id(&ids_str).await);
                },
                "titres" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_titres_user_id(&ids_str).await);
                },
                "specialites" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_specialites_user_id(&ids_str).await);
                },
                "domaines" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_domaines_user_id(&ids_str).await);
                },
                "departements" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_department_user_id(&ids_str).await);
                },
                "langues" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_langues_user_id(&ids_str).await);
                },

                "localites" if ids.len() > 0 => {
                    result.extend_from_slice(&self.user_localites_user_id(&ids_str).await);
                },
                _ => {}
            }
        }

        result.into_iter().collect()
    }
    
    async fn get_related_tables(&self, msg: &AnnuaireSearchInput) -> BTreeMap<String, Vec<RowId>> {
        let mut result: BTreeMap<String, Vec<RowId>> = BTreeMap::new();
        
        match &msg.key {
            Some(key) => {
                match self.search_campus_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("campus".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("Campus: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_user_plus_infos_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("user_plus_infos".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("user_plus_infos: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_competence_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("competences".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("competences: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_diplomes_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("diplomes_certificats".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("diplomes_certificats: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_entreprises_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("entreprises".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("entreprises: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_ecoles_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("ecoles".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("ecoles: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_titres_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("titres".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("titres: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_specialites_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("specialites".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("specialites: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.domaines_search_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("domaines".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("domaines: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_departements_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("departements".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("departements: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_langues_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("langues".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("langues: {e:#?}");
                    }
                }
            },
            None => {}
        }

        match &msg.key {
            Some(key) => {
                match self.search_localites_by_key(&key).await {
                    Ok(res) => {
                        if res.len() > 0 {
                            result.insert("localites".to_string(), res);
                        }
                    },
                    Err(e) => {
                        println!("localites: {e:#?}");
                    }
                }
            },
            None => {}
        }

        result
    }

    
    async fn get_users(&self, ids: &HashSet<RowId>, church: Option<&str>) -> Vec<User> {
        let filter_campus: Vec<i32> = match church {
            Some(ch) => {
                match self.search_campus_by_key(&ch).await {
                    Ok(res) => {
                        res.iter().filter(|r| r.id.is_some()).map(|r| r.id.unwrap()).collect()
                    },
                    Err(_) => vec![]
                }
            },
            None => vec![]
        };

        let users_ids: Vec<_> = ids.iter().filter(|r| r.id.is_some()).map(|r| r.id.unwrap()).collect();
        let mut users = self.users_by_ids(&users_ids, &filter_campus).await;
        //println!("users: {users:?}");
        for user in &mut users {
            user.campus = self.get_campus_by_user_id(&user.id.unwrap_or_default()).await;
            user.departements = self.get_departements_by_user_id(&user.id.unwrap_or_default()).await;
            user.contact = self.contact_by_user_id(&user.id.unwrap_or_default()).await;
            user.competences = self.get_competence_by_user_id(&user.id.unwrap_or_default()).await;
            user.diplomes = self.get_diplomes_by_user_id(&user.id.unwrap_or_default()).await;
            user.domaines = self.domaines_by_user_id(&user.id.unwrap_or_default()).await;
            user.ecoles = self.get_ecoles_by_user_id(&user.id.unwrap_or_default()).await;
            user.entreprises = self.get_entreprises_by_user_id(&user.id.unwrap_or_default()).await;
            user.langues = self.get_langues_by_user_id(&user.id.unwrap_or_default()).await;
            user.localites = self.get_localites_by_user_id(&user.id.unwrap_or_default()).await;
            user.specialites = self.get_specialites_by_user_id(&user.id.unwrap_or_default()).await;
            user.titres = self.get_titres_by_user_id(&user.id.unwrap_or_default()).await;
            user.user_plus_infos = self.get_user_plus_infos_by_user_id(&user.id.unwrap_or_default()).await;
        }

        users
    }

    pub(crate) async fn save_query(&self, sql: &str) -> i32 {
        match sqlx::query_as::<_, RowId>(&sql)
        .fetch_one(&self.pool)
        .await 
        {
            Ok(res) => {
                match res.id {
                    Some(id) => id,
                    None => 0
                }
            },
            Err(e) => {
                println!("err in inserting user: {e:#?}");
                0
            }
        }
    }
}