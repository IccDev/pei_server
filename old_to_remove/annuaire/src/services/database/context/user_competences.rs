use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;


impl DatabaseService {
    pub async fn user_competences_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_competences_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_competences: {e:#?}");
                vec![]
            }
        }
    }
    
    pub async fn create_user_competences(&self, user_id: &i32, competences_id: &i32, consentement: bool) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_competences (id_user, id_competence, consentement) 
            values ({}, {}, {})
            returning id;
            "#,
            user_id,
            competences_id,
            consentement
        ).as_ref()).await
    }

    fn user_competences_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_competences uc 
        where uc.id_competence in ({msg});
        "#)
    }

}