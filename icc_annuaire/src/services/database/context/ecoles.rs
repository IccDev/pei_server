use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, Ecole}};


impl DatabaseService {

    pub(crate) async fn ecoles_by_user_id(&self, id: &i32) -> Vec<Ecole> {
    
        match sqlx::query_as::<_, Ecole>(&self.ecoles_by_user_id_sql(&id).as_str())
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

    fn ecoles_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT d.id, d.nom, COALESCE(d.description, '') as description, COALESCE(ue.consentement, 'false') as consentement
        FROM annuaire.ecoles d
        join annuaire.user_ecoles ue
        on d.id = ue.id_ecole 
        where ue.id_user = {id};
        "#)
    }
}