use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Competence};


impl DatabaseService {

    pub(crate) async fn competence(&self) -> Vec<Competence> {
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

    pub(crate) async fn competence_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
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

    pub(crate) async fn competence_by_user_id(&self, user_id: &i32) -> Vec<Competence> {
        match sqlx::query_as::<_, Competence>(&self.competence_by_user_id_sql(&user_id).as_ref())
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

    fn competence_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT c.id, COALESCE(c.nom, null) as nom, COALESCE(c.description, null) as description
        FROM annuaire.competences c
        join annuaire.user_competences uc 
        on c.id = uc.id_competence
        WHERE uc.id_user = {user_id};"#)
    }
}