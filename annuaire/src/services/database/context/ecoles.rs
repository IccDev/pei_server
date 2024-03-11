use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Ecole, Localite};


impl DatabaseService {

    pub async fn get_all_ecoles(&self) -> Vec<Ecole> {
    
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

    pub async fn create_ecole(&self, id_localite: i32, ecole: &Ecole) -> i32 {
        self.save_query(&self.save_ecole_sql(id_localite, &ecole)).await
    }

    pub async fn save_ecole(&self, ecole: &Ecole, localite: &Localite) -> i32 {
        let locale_id = self.create_localite(&localite).await;
        self.save_query(&self.save_ecole_sql(locale_id, &ecole)).await
    }

    pub async fn get_ecoles_by_user_id(&self, user_id: &i32) -> Vec<Ecole> {
        match sqlx::query_as::<_, Ecole>(&self.get_ecoles_by_user_id_sql(&user_id).as_ref())
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

    pub async fn search_ecoles_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
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

    fn get_ecoles_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT e.id, e.id_localite, COALESCE(e.nom, '') as nom, COALESCE(e.description, '') as description, COALESCE(ue.consentement, 'false') as consentement
        FROM annuaire.ecoles e
        join annuaire.user_ecoles ue 
        on e.id = ue.id_ecole
        WHERE ue.id_user = {user_id};"#)
    }

    fn save_ecole_sql(&self, id_localite: i32, ecole: &Ecole) -> String {
        format!(r#"
            insert into annuaire.ecoles 
                (id_localite, nom, description) 
            values 
                ({}, '{}', '{}') 
            returning id
        "#,
        id_localite,
        ecole.nom.clone().unwrap_or_default(),
        ecole.description.clone().unwrap_or_default()
        )
    }
}