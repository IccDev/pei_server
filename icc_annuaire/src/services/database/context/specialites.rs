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
    annuaire::{AnnuaireSearch, Specialite}
};


impl DatabaseService {

    pub(crate) async fn specialites_by_user_id(&self, id: &i32) -> Vec<Specialite> {
    
        match sqlx::query_as::<_, Specialite>(&self.specialites_by_user_id_sql(&id).as_str())
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

    fn specialites_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT s.id, s.nom, COALESCE(s.description, '') as description
        FROM annuaire.specialites s
        join annuaire.user_specialites us
        on s.id = us.id_specialite
        where us.id_user = {id};
        "#)
    }
}