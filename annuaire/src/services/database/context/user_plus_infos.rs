use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, UserPlusInfos};


impl DatabaseService {

    pub(crate) async fn user_plus_infos(&self) -> Vec<UserPlusInfos> {
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

    pub(crate) async fn user_plus_infos_by_user_id(&self, user_id: &i32) -> Option<UserPlusInfos> {
        match sqlx::query_as::<_, UserPlusInfos>(&self.user_plus_infos_by_user_id_sql(&user_id))
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

    pub(crate) async fn user_plus_infos_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
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

    fn user_plus_infos_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT upi.id, COALESCE(upi.description, null) as description
        FROM annuaire.user_plus_infos upi
        WHERE upi.id_user = {user_id}"#)
    }
}