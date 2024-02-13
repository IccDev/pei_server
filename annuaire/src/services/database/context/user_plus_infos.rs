use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, UserPlusInfos}};


impl DatabaseService {

    pub(crate) async fn user_plus_infos_by_user_id(&self, id: &i32) -> Vec<UserPlusInfos> {
        match sqlx::query_as::<_, UserPlusInfos>(&self.user_plus_infos_by_user_id_sql(&id).as_str())
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

    fn user_plus_infos_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT upi.id, COALESCE(upi.description, '') as description
        FROM annuaire.user_plus_infos upi
        where upi.id_user = {id};
        "#)
    }
}