use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;


impl DatabaseService {
    pub async fn user_titres_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_titres_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_titres: {e:#?}");
                vec![]
            }
        }
    }

    fn user_titres_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_titres uc 
        where uc.id_titre in ({msg});
        "#)
    }
}