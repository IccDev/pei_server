use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Entreprise};


impl DatabaseService {

    pub(crate) async fn entreprises(&self) -> Vec<Entreprise> {
    
        match sqlx::query_as::<_, Entreprise>(&self.entreprise_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in entreprise: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn entreprises_by_user_id(&self, user_id: &i32) -> Vec<Entreprise> {
        match sqlx::query_as::<_, Entreprise>(&self.entreprise_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in entreprise: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn entreprises_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("entreprises", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in entreprises search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn entreprise_sql(&self) -> &str {
        r#"SELECT e.id, e.id_localite, COALESCE(e.nom, null) as nom, COALESCE(e.description, null) as description
        FROM annuaire.entreprises e"#
    }

    fn entreprise_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT e.id, e.id_localite, COALESCE(e.nom, null) as nom, COALESCE(e.description, null) as description
        FROM annuaire.entreprises e
        join annuaire.user_entreprises ue 
        on e.id = ue.id_entreprise
        WHERE ue.id_user = {user_id};"#)
    }
}