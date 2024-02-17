use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Departement};


impl DatabaseService {

    pub(crate) async fn departements(&self) -> Vec<Departement> {
    
        match sqlx::query_as::<_, Departement>(&self.departements_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(mut res) => {
                for r in &mut *res {
                    r.campus = self.campus_by_id(r.id_campus.as_ref()).await;
                }
                res.to_vec()
            },
            Err(e) => {
                println!("err in departement: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn departements_by_user_id(&self, user_id: &i32) -> Vec<Departement> {
    
        match sqlx::query_as::<_, Departement>(&self.departements_by_user_id_sql(&user_id))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(mut res) => {
                for r in &mut *res {
                    r.campus = self.campus_by_id(r.id_campus.as_ref()).await;
                }
                res.to_vec()
            },
            Err(e) => {
                println!("err in departement: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn departements_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("departements", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in departements search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn departements_sql(&self) -> &str {
        r#"SELECT d.id, d.id_campus, COALESCE(d.nom, '') as nom, COALESCE(d.abbreviation, '') as abbreviation, COALESCE(d.description, '') as description
        FROM annuaire.departements d"#
    }

    fn departements_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT d.id, d.id_campus, COALESCE(d.nom, '') as nom, COALESCE(d.abbreviation, '') as abbreviation, COALESCE(d.description, '') as description
            FROM annuaire.departements d
            join annuaire.user_department ud 
            on d.id = ud.id_departement
            WHERE ud.id_user = {user_id};
        "#)
    }
}