use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, Domaine}};


impl DatabaseService {

    pub(crate) async fn domaines_by_user_id(&self, id: &i32) -> Vec<Domaine> {
    
        match sqlx::query_as::<_, Domaine>(&self.domaines_by_user_id_sql(&id).as_str())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in domaines: {e:#?}");
                vec![]
            }
        }
    }

    fn domaines_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT d.id, d.nom, COALESCE(d.description, '') as description
        FROM annuaire.domaines d
        join annuaire.user_domaines ud
        on d.id = ud.id_domaine 
        where ud.id_user = {id};
        "#)
    }
}