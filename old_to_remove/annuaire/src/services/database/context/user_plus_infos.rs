use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, UserPlusInfos};


impl DatabaseService {

    pub async fn user_plus_infos(&self) -> Vec<UserPlusInfos> {
        match sqlx::query_as::<_, UserPlusInfos>(&self.user_plus_infos_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_plus_infos: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn save_user_plus_infos(&self, info: &UserPlusInfos) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_plus_infos (id_user, description) 
            values ({}, '{}') 
            returning id;
            "#,
            info.id_user.clone().unwrap_or_default(),
            info.description.clone().unwrap_or_default()
        ).as_ref()).await
    }

    pub async fn create_user_plus_infos(&self, user_id: &i32, info: &UserPlusInfos) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_plus_infos (id_user, description) 
            values ({}, '{}') 
            returning id;
            "#,
            user_id,
            info.description.clone().unwrap_or_default()
        ).as_ref()).await
    }

    pub async fn get_user_plus_infos_by_user_id(&self, user_id: &i32) -> Option<UserPlusInfos> {
        match sqlx::query_as::<_, UserPlusInfos>(&self.get_user_plus_infos_by_user_id_sql(&user_id))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.first().cloned()
            },
            Err(e) => {
                println!("err in user_plus_infos: {e:#?}");
                None
            }
        }
    }

    fn user_plus_infos_sql(&self) -> &str {
        r#"SELECT upi.id, COALESCE(upi.description, null) as description
        FROM annuaire.user_plus_infos upi"#
    }

    pub async fn search_user_plus_infos_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("user_plus_infos", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in user_plus_infos search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn get_user_plus_infos_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT upi.id, COALESCE(upi.description, null) as description
        FROM annuaire.user_plus_infos upi
        WHERE upi.id_user = {user_id}"#)
    }
}