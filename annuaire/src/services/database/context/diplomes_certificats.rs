use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, DiplomeCertificat}};


impl DatabaseService {

    pub(crate) async fn diplomes_by_user_id(&self, id: &i32) -> Vec<DiplomeCertificat> {
    
        match sqlx::query_as::<_, DiplomeCertificat>(&self.diplomes_by_user_id_sql(&id).as_str())
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

    fn diplomes_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT d.id, d.nom, COALESCE(d.description, '') as description
        FROM annuaire.diplomes_certificats d
        join annuaire.user_diplomes ud
        on d.id = ud.id_diplome 
        where ud.id_user = {id};
        "#)
    }
}