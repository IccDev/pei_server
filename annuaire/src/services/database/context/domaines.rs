use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Domaine};


impl DatabaseService {

    pub(crate) async fn _domaines(&self) -> Vec<Domaine> {
    
        match sqlx::query_as::<_, Domaine>(&self.domaines_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in domaines: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn domaines_by_user_id(&self, user_id: &i32) -> Vec<Domaine> {
        match sqlx::query_as::<_, Domaine>(&self.domaines_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in domaines: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn domaines_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("domaines", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in domaines search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn domaines_sql(&self) -> &str {
        r#"SELECT d.id, COALESCE(d.nom, null) as nom, COALESCE(d.description, null) as description
        FROM annuaire.domaines d"#
    }

    fn domaines_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT d.id, COALESCE(d.nom, null) as nom, COALESCE(d.description, null) as description
        FROM annuaire.domaines d
        join annuaire.user_domaines ud 
        on d.id = ud.id_domaine
        WHERE ud.id_user = {user_id};"#)
    }
}