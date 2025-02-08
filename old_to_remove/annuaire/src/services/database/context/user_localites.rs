use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;

impl DatabaseService {
    pub async fn user_localites_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_localites_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_localites: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_user_localite(&self, user_id: &i32, id_localite: &i32, consentement: &bool) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_localites (id_localite, id_user, consentement) 
            values ({}, {}, {})
            returning id;
            "#,
            id_localite,
            user_id,
            consentement
        ).as_ref()).await
    }

    fn user_localites_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_localites uc 
        where uc.id_localite in ({msg});
        "#)
    }
}