use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Ecole};


impl DatabaseService {

    pub(crate) async fn ecoles(&self) -> Vec<Ecole> {
    
        match sqlx::query_as::<_, Ecole>(&self.ecoles_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in ecoles: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn ecoles_by_user_id(&self, user_id: &i32) -> Vec<Ecole> {
        match sqlx::query_as::<_, Ecole>(&self.ecoles_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in ecoles: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn ecoles_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("ecoles", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in ecoles search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn ecoles_sql(&self) -> &str {
        r#"SELECT e.id, e.id_localite, COALESCE(e.nom, '') as nom, COALESCE(e.description, '') as description, COALESCE(ue.consentement, 'false') as consentement
        FROM annuaire.ecoles e"#
    }

    fn ecoles_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT e.id, e.id_localite, COALESCE(e.nom, '') as nom, COALESCE(e.description, '') as description, COALESCE(ue.consentement, 'false') as consentement
        FROM annuaire.ecoles e
        join annuaire.user_ecoles ue 
        on e.id = ue.id_ecole
        WHERE ue.id_user = {user_id};"#)
    }
}