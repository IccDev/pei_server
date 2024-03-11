use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;

impl DatabaseService {
    pub async fn user_entreprises_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_entreprises_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_entreprises: {e:#?}");
                vec![]
            }
        }
    }
    
    pub async fn create_user_entreprise(&self, user_id: &i32, id_entreprise: &i32, consentement: &bool) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_entreprises (id_entreprise, id_user, consentement) 
            values ({}, {}, {})
            returning id;
            "#,
            id_entreprise,
            user_id,
            consentement
        ).as_ref()).await
    }

    fn user_entreprises_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_entreprises uc 
        where uc.id_entreprise in ({msg});
        "#)
    }
}