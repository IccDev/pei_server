use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Competence};


impl DatabaseService {

    pub async fn get_all_competences(&self) -> Vec<Competence> {
        match sqlx::query_as::<_, Competence>(&self.competence_sql())
        .fetch_all(&self.pool)
        .await
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in competences: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_competence(&self, competence: &Competence) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.competences (nom, description) 
            values ('{}', '{}') 
            returning id;
            "#,
            competence.nom.clone().unwrap_or_default(),
            competence.description.clone().unwrap_or_default()
        ).as_ref()).await
    }

    pub async fn search_competence_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("competences", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in competence search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    pub async fn get_competence_by_user_id(&self, user_id: &i32) -> Vec<Competence> {
        match sqlx::query_as::<_, Competence>(&self.get_competence_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in Competence: {e:#?}");
                vec![]
            }
        }
    }

    fn competence_sql(&self) -> &str {
        r#"SELECT c.id, COALESCE(c.nom, null) as nom, COALESCE(c.description, null) as description
        FROM annuaire.competences c;"#
    }

    fn get_competence_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT c.id, COALESCE(c.nom, null) as nom, COALESCE(c.description, null) as description
        FROM annuaire.competences c
        join annuaire.user_competences uc 
        on c.id = uc.id_competence
        WHERE uc.id_user = {user_id};"#)
    }
}