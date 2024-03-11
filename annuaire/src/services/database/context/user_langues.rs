use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;

impl DatabaseService {
    pub async fn user_langues_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_langues_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_langues: {e:#?}");
                vec![]
            }
        }
    }
    
    pub async fn create_user_langue(&self, user_id: &i32, id_langues: &i32) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_langues (id_langues, id_user) 
            values ({}, {})
            returning id;
            "#,
            id_langues,
            user_id
        ).as_ref()).await
    }

    fn user_langues_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_langues uc 
        where uc.id_langues in ({msg});
        "#)
    }
}