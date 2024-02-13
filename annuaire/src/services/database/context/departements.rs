use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, Departement}};


impl DatabaseService {

    pub(crate) async fn departements_by_user_id(&self, id: &i32) -> Vec<Departement> {
    
        match sqlx::query_as::<_, Departement>(&self.departements_by_user_id_sql(&id).as_str())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in departement: {e:#?}");
                vec![]
            }
        }
    }

    fn departements_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT d.id, d.nom, COALESCE(d.abbreviation, '') as abbreviation, COALESCE(d.description, '') as description
        FROM annuaire.departements d
        join annuaire.user_department ud
        on d.id = ud.id_departement 
        where ud.id_user = {id};
        "#)
    }
}