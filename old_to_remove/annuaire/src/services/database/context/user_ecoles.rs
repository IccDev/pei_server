use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;


impl DatabaseService {
    pub async fn user_ecoles_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_ecoles_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_ecoles: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_user_ecole(&self, user_id: &i32, id_ecole: &i32, consentement: &bool) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_ecoles (id_ecole, id_user, consentement) 
            values ({}, {}, {})
            returning id;
            "#,
            id_ecole,
            user_id,
            consentement
        ).as_ref()).await
    }

    fn user_ecoles_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_ecoles uc 
        where uc.id_ecole in ({msg});
        "#)
    }
}