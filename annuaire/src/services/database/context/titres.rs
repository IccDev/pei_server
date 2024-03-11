use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Titre};


impl DatabaseService {

    pub async fn get_titres(&self) -> Vec<Titre> {
    
        match sqlx::query_as::<_, Titre>(&self.titres_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in titres: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_titre(&self, titre: &Titre) -> i32 {
        self.save_query(&self.save_titre_sql(&titre)).await
    }

    pub async fn get_titres_by_user_id(&self, user_id: &i32) -> Vec<Titre> {
        match sqlx::query_as::<_, Titre>(&self.get_titres_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in titres: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn search_titres_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("titres", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in titres search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn titres_sql(&self) -> &str {
        r#"SELECT t.id, t.nom, COALESCE(t.description, '') as description
        FROM annuaire.titres t"#
    }

    fn get_titres_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT t.id, t.nom, COALESCE(t.description, '') as description
        FROM annuaire.titres t
        join annuaire.user_titres ut
        on t.id = ut.id_titre
        WHERE ut.id_user = {user_id}"#)
    }

    fn save_titre_sql(&self, titre: &Titre) -> String {
        format!(r#"
            insert into annuaire.titres 
                (nom, description) 
            values 
                ('{}', '{}') 
            returning id
        "#,
        titre.nom.clone().unwrap_or_default(),
        titre.description.clone().unwrap_or_default()
        )
    }
}