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
    annuaire::{AnnuaireSearch, Titre}
};


impl DatabaseService {

    pub(crate) async fn titres_by_user_id(&self, id: &i32) -> Vec<Titre> {
    
        match sqlx::query_as::<_, Titre>(&self.titres_by_user_id_sql(&id).as_str())
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

    fn titres_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT t.id, t.nom, COALESCE(t.description, '') as description
        FROM annuaire.titres t
        join annuaire.user_titres ut
        on t.id = ut.id_titre
        where ut.id_user = {id};
        "#)
    }
}