use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Langue};


impl DatabaseService {

    pub async fn get_all_langues(&self) -> Vec<Langue> {
    
        match sqlx::query_as::<_, Langue>(&self.langues_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in langues: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_langue(&self, langue: &Langue) -> i32 {
        self.save_query(&self.save_langue_sql(&langue)).await
    }

    pub async fn get_langues_by_user_id(&self, user_id: &i32) -> Vec<Langue> {
    
        match sqlx::query_as::<_, Langue>(&self.get_langues_by_user_id_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in langues: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn search_langues_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("langues", &key))
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

    fn langues_sql(&self) -> &str {
        r#"SELECT l.id, COALESCE(l.nom, null) as nom, COALESCE(l.abbreviation, null) as abbreviation
        FROM annuaire.langues l"#
    }

    fn get_langues_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT l.id, COALESCE(l.nom, null) as nom, COALESCE(l.abbreviation, null) as abbreviation
        FROM annuaire.langues l
        join annuaire.user_langues ul 
        on l.id = ul.id_langues
        WHERE ul.id_user = {user_id};"#)
    }

    fn save_langue_sql(&self, langue: &Langue) -> String {
        format!(r#"
            insert into annuaire.langues 
                (nom, abbreviation) 
            values 
                ('{}', '{}') 
            returning id
        "#,
        langue.nom.clone().unwrap_or_default(),
        langue.abbreviation.clone().unwrap_or_default()
        )
    }
}