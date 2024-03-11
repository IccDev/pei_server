use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, DiplomeCertificat};


impl DatabaseService {

    pub async fn get_all_diplomes(&self) -> Vec<DiplomeCertificat> {
        match sqlx::query_as::<_, DiplomeCertificat>(&self.diplomes_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in diplomes: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_diplome(&self, diplome: &DiplomeCertificat) -> i32 {
        self.save_query(
            format!(r#"insert into annuaire.diplomes_certificats (nom, description) 
            values ('{}', '{}') 
            returning id;
            "#,
            diplome.nom.clone().unwrap_or_default(),
            diplome.description.clone().unwrap_or_default()
        ).as_ref()).await
    }

    pub async fn get_diplomes_by_user_id(&self, user_id: &i32) -> Vec<DiplomeCertificat> {
        match sqlx::query_as::<_, DiplomeCertificat>(&self.diplomes_user_id_sql(&user_id))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in diplomes: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn search_diplomes_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
        match sqlx::query_as::<_, RowId>(&self.search_in_table("diplomes_certificats", &key))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                Ok(res.to_vec())
            },
            Err(e) => {
                println!("err in diplomes_certificats search key: {e:#?}");
                Err(format!("{e:#?}"))
            }
        }
    }

    fn diplomes_sql(&self) -> &str {
        r#"SELECT d.id, COALESCE(d.nom, null) as nom, COALESCE(d.description, null) as description
        FROM annuaire.diplomes_certificats d"#
    }

    fn diplomes_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT d.id, COALESCE(d.nom, null) as nom, COALESCE(d.description, null) as description
        FROM annuaire.diplomes_certificats d
        join annuaire.user_diplomes ud 
        on d.id = ud.id_diplome
        WHERE ud.id_user = {user_id};"#)
    }
}