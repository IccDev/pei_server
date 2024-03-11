use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::RowId;

impl DatabaseService {
    pub async fn user_department_user_id(&self, msg: &str) -> Vec<RowId> {
        match sqlx::query_as::<_, RowId>(&self.user_department_user_id_sql(&msg))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in user_department: {e:#?}");
                vec![]
            }
        }
    }
    
    pub async fn create_user_department(&self, user_id: &i32, campus_id: &i32, id_departement: &i32) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.user_department (id_campus, id_user, id_departement) 
            values ({}, {}, {})
            returning id;
            "#,
            campus_id,
            user_id,
            id_departement
        ).as_ref()).await
    }

    fn user_department_user_id_sql(&self, msg: &str) -> String {
        format!(r#"
        SELECT uc.id_user as id
        FROM annuaire.user_department uc 
        where uc.id_departement in ({msg});
        "#)
    }
}