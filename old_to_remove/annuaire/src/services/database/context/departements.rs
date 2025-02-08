use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Departement, Campus};


impl DatabaseService {

    pub async fn get_all_departements(&self) -> Vec<Departement> {
    
        match sqlx::query_as::<_, Departement>(&self.departements_sql())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(mut res) => {
                for r in &mut *res {
                    r.campus = self.get_campus_by_id(r.id_campus.as_ref()).await;
                }
                res.to_vec()
            },
            Err(e) => {
                println!("err in departement: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn create_departement(&self, id_compus: i32, departement: &Departement) -> i32 {
        self.save_query(&self.create_departement_sql(id_compus, &departement)).await
    }

    pub async fn save_departement(&self, locale_id: &i32, campus: &Campus, departement: &Departement) -> i32 {
        let id_campus = self.create_campus(&locale_id, &campus).await;
        self.save_query(&self.create_departement_sql(id_campus, &departement)).await
    }

    pub async fn get_departements_by_user_id(&self, user_id: &i32) -> Vec<Departement> {
    
        match sqlx::query_as::<_, Departement>(&self.get_departements_by_user_id_sql(&user_id))
        .fetch_all(&self.pool)
        .await 
        {
            Ok(mut res) => {
                for r in &mut *res {
                    r.campus = self.get_campus_by_id(r.id_campus.as_ref()).await;
                }
                res.to_vec()
            },
            Err(e) => {
                println!("err in departement: {e:#?}");
                vec![]
            }
        }
    }

    pub async fn search_departements_by_key(&self, key: &str) -> Result<Vec<RowId>, String> {
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

    fn get_departements_by_user_id_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT d.id, d.id_campus, COALESCE(d.nom, '') as nom, COALESCE(d.abbreviation, '') as abbreviation, COALESCE(d.description, '') as description
            FROM annuaire.departements d
            join annuaire.user_department ud 
            on d.id = ud.id_departement
            WHERE ud.id_user = {user_id};
        "#)
    }

    fn create_departement_sql(&self, id_campus: i32, departement: &Departement) -> String {
        format!(r#"
            insert into annuaire.departements 
                (id_campus, nom, abbreviation, description) 
            values 
                ({}, '{}', '{}', '{}') 
            returning id
        "#,
        id_campus,
        departement.nom.clone().unwrap_or_default(),
        departement.abbreviation.clone().unwrap_or_default(),
        departement.description.clone().unwrap_or_default()
        )
    }
}