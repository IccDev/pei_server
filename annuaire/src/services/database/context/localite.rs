use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Localite};


impl DatabaseService {

    pub(crate) async fn _localites(&self) -> Vec<Localite> {
    
        match sqlx::query_as::<_, Localite>(&self.localite_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in localite: {e:#?}");
                vec![]
            }
        }
    }

    pub(crate) async fn localites_by_user_id(&self, user_id: &i32) -> Option<Localite> {
    
        match sqlx::query_as::<_, Localite>(&self.localite_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.first().cloned()
            },
            Err(e) => {
                println!("err in localite: {e:#?}");
                None
            }
        }
    }

    pub(crate) async fn localite_by_id(&self, id: Option<&i32>) -> Option<Localite> {
        match id {
            Some(i) => {
                match sqlx::query_as::<_, Localite>(&self.localite_by_id_sql(&i))
                    .fetch_all(&self.pool)
                    .await 
                    {
                        Ok(res) => {
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

    pub(crate) async fn localites_search_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("localites", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in localites search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn localite_sql(&self) -> &str {
        r#"SELECT 
            l.id, 
            COALESCE(l.pays, '') as pays,
            COALESCE(l.ville, '') as ville,
            COALESCE(l.code_postal, '') as code_postal,
            COALESCE(l.commune, '') as commune,
            COALESCE(l.quartier, '') as quartier,
            COALESCE(l.adresse, '') as adresse,
            COALESCE(ul.consentement, 'false') as consentement
        FROM annuaire.localites l
        join annuaire.user_localites ul
        on l.id = ul.id_localite"#
    }

    fn localite_by_id_sql(&self, id: &i32) -> String {
        format!(r#"SELECT 
            l.id, 
            COALESCE(l.pays, '') as pays,
            COALESCE(l.ville, '') as ville,
            COALESCE(l.code_postal, '') as code_postal,
            COALESCE(l.commune, '') as commune,
            COALESCE(l.quartier, '') as quartier,
            COALESCE(l.adresse, '') as adresse,
            false as consentement
        FROM annuaire.localites l
        WHERE l.id = {id}"#)
    }

    fn localite_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT 
            l.id, 
            COALESCE(l.pays, '') as pays,
            COALESCE(l.ville, '') as ville,
            COALESCE(l.code_postal, '') as code_postal,
            COALESCE(l.commune, '') as commune,
            COALESCE(l.quartier, '') as quartier,
            COALESCE(l.adresse, '') as adresse,
            COALESCE(ul.consentement, 'false') as consentement
        FROM annuaire.localites l
        join annuaire.user_localites ul
        on l.id = ul.id_localite
        WHERE ul.id_user = {user_id}"#)
    }
}