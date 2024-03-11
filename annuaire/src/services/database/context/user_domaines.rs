use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;

impl DatabaseService {
    pub async fn user_domaines_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_domaines_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_domaines: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_user_domaine(&self, user_id: &i32, id_domaine: &i32) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_domaines (id_user, id_domaine) 
            values ({}, {})
            returning id;
            "#,
            user_id,
            id_domaine
        ).as_ref()).await
    }

    fn user_domaines_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_domaines uc 
        where uc.id_domaine in ({msg});
        "#)
    }
}