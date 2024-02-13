use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{
    ResponseData, 
    annuaire::{AnnuaireSearch, Entreprise}
};


impl DatabaseService {

    pub(crate) async fn entreprises_by_user_id(&self, id: &i32) -> Vec<Entreprise> {
    
        match sqlx::query_as::<_, Entreprise>(&self.entreprise_by_user_id_sql(&id).as_str())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in entreprise: {e:#?}");
                vec![]
            }
        }
    }

    fn entreprise_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT e.id, e.nom, COALESCE(e.description, '') as description
        FROM annuaire.entreprises e
        join annuaire.user_entreprises ue
        on e.id = ue.id_entreprise 
        where ue.id_user = {id};
        "#)
    }
}