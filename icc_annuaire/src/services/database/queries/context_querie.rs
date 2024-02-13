use crate::{
    services::{
        database::AnnuaireTableActor
    }
};
use icc_common::{
    acteur::{Respond, ActorAssistant},
    async_trait,
    sqlx
};
use inter_services_messages::{annuaire::{RowId, AnnuaireSearchInput, AnnuaireSearch, AnnuaireSearchResponse}};
use std::collections::BTreeMap;

#[async_trait::async_trait] 
impl Respond<AnnuaireSearch> for AnnuaireTableActor {
    type Response = Result<AnnuaireSearchResponse, String>;

    async fn handle(&mut self, message: AnnuaireSearch, system: &ActorAssistant<Self>) -> Self::Response {
        self.search_by_table(message, &system).await
    }
}

impl AnnuaireTableActor {
    pub(crate) async fn search_by_table(&mut self, msg: AnnuaireSearch, _system: &ActorAssistant<Self>) -> Result<AnnuaireSearchResponse, String> {
        
        match &msg {
            AnnuaireSearch::ByKey(annuaire_search_input) => {
                match (self.table.as_ref(), &annuaire_search_input.key, &annuaire_search_input.church) {
                    ("campus", _ ,Some(church_value)) => {
                        match sqlx::query_as::<_, RowId>(search_in_table_by_key(&self.table, &church_value).as_str())
                                .fetch_all(&self.pool)
                                .await 
                                {
                                    Ok(res) => {
                                        Ok(AnnuaireSearchResponse::ByKeyResponse((self.table.clone(), res.to_vec())))
                                    },
                                    Err(e) => {
                                        println!("err: {e:#?}");
                                        Err("there is church".to_owned())
                                    }
                                }
                    },
                    (_, Some(key_value), _) => {
                        match sqlx::query_as::<_, RowId>(search_in_table_by_key(&self.table, &key_value).as_str())
                                .fetch_all(&self.pool)
                                .await 
                                {
                                    Ok(res) => {
                                        Ok(AnnuaireSearchResponse::ByKeyResponse((self.table.clone(), res.to_vec())))
                                    },
                                    Err(e) => {
                                        println!("err: {e:#?}");
                                        Err("there is key".to_owned())
                                    }
                                }
                    },
                    (_, _, _) => Err("there are key and church".to_owned())
                }
            },
            AnnuaireSearch::ByIds((column, ids)) => {
                let ids_string: Vec<String> = ids.iter().map(|i| format!("{i}")).collect();

                match sqlx::query_as::<_, RowId>(search_users_ids(&self.table, &column, &ids_string.as_slice().join(", ")).as_str())
                                .fetch_all(&self.pool)
                                .await 
                                {
                                    Ok(res) => {
                                        Ok(AnnuaireSearchResponse::UserIds(res.to_vec()))
                                    },
                                    Err(e) => {
                                        println!("err: {e:#?}");
                                        Err("there is church".to_owned())
                                    }
                                }
            },
            _ => Err("expecting search by key and church!".to_owned())
        }
    }
}

fn search_in_table_by_key(table: &str, key: &str) -> String {
    format!(r#"
        SELECT t.id
        FROM annuaire.{table} t
        WHERE tsv @@ to_tsquery('pg_catalog.english', '"{key}"');
    "#)
}

fn search_users_ids(table: &str, column: &str, ids: &str) -> String {
    format!(r#"
        SELECT t.id_user as id
        FROM annuaire.{table} t
        WHERE t.{column} in ({ids});
    "#)
}