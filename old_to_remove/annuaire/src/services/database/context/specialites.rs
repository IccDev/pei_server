use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Specialite};


impl DatabaseService {

    pub async fn get_all_specialites(&self) -> Vec<Specialite> {
    
        match sqlx::query_as::<_, Specialite>(&self.specialites_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in specialites: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_specialite(&self, specialite: &Specialite) -> i32 {
        self.save_query(&self.save_specialite_sql(&specialite)).await
    }

    pub async fn get_specialites_by_user_id(&self, user_id: &i32) -> Vec<Specialite> {
    
        match sqlx::query_as::<_, Specialite>(&self.get_specialites_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in specialites: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn search_specialites_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("specialites", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in langues search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn specialites_sql(&self) -> &str {
        r#"SELECT s.id, COALESCE(s.nom, '') as nom, COALESCE(s.description, '') as description
        FROM annuaire.specialites s"#
    }

    fn get_specialites_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT s.id, COALESCE(s.nom, '') as nom, COALESCE(s.description, '') as description
        FROM annuaire.specialites s
        join annuaire.user_specialites us
        on s.id = us.id_specialite
        WHERE us.id_user = {user_id}"#)
    }

    fn save_specialite_sql(&self, specialite: &Specialite) -> String {
        format!(r#"
            insert into annuaire.specialites 
                (nom, description) 
            values 
                ('{}', '{}') 
            returning id
        "#,
        specialite.nom.clone().unwrap_or_default(),
        specialite.description.clone().unwrap_or_default()
        )
    }
}