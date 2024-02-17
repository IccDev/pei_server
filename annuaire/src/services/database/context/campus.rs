use crate::services::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{Campus, RowId};


impl DatabaseService {

    pub(crate) async fn _campus(&self) -> Vec<Campus> {
        match sqlx::query_as::<_, Campus>(&self.campus_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(mut res) => {
                for r in &mut *res {
                    r.localite = self.localite_by_id(r.id_localite.as_ref()).await;
                }
                res.to_vec()
            },
            Err(e) => {
                println!("err in campus: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn campus_by_user_id(&self, user_id: &i32) -> Vec<Campus> {
        match sqlx::query_as::<_, Campus>(&self.campus_by_user_id_sql(&user_id))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(mut res) => {
                for r in &mut *res {
                    r.localite = self.localite_by_id(r.id_localite.as_ref()).await;
                }
                res.to_vec()
            },
            Err(e) => {
                println!("err in campus: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn campus_by_id(&self, id: Option<&i32>) -> Option<Campus> {
        match id {
            Some(i) => {
                match &mut sqlx::query_as::<_, Campus>(&self.campus_by_id_sql(&i))
                .fetch_all(&self.pool)
                .await 
                {
                    Ok(res) => {
                        for r in &mut *res {
                            r.localite = self.localite_by_id(r.id_localite.as_ref()).await;
                        }
                        res.first().cloned()
                    },
                    Err(e) => {
                        println!("err in campus: {e:#?}");
                        None
                    }
                }
            },
            None => None
        }
    }

    pub(crate) async fn campus_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("campus", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in campus search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn campus_sql(&self) -> &str {
        r#"SELECT c.id, c.id_localite, COALESCE(c.nom, '') as nom, COALESCE(c.description, '') as description
        FROM annuaire.campus c"#
    }

    fn campus_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT c.id, c.id_localite, COALESCE(c.nom, '') as nom, COALESCE(c.description, '') as description
            FROM annuaire.campus c
            join annuaire.user_campus uc 
            on c.id = uc.id_campus
            WHERE uc.id_user = {user_id};
        "#)
    }

    fn campus_by_id_sql(&self, id: &i32) -> String {
        format!(r#"SELECT c.id, c.id_localite, COALESCE(c.nom, '') as nom, COALESCE(c.description, '') as description
            FROM annuaire.campus c
            WHERE c.id = {id};
        "#)
    }
}