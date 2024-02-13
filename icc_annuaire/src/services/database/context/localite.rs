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
    annuaire::{AnnuaireSearch, Localite}
};


impl DatabaseService {

    pub(crate) async fn localites_by_user_id(&self, id: &i32) -> Vec<Localite> {
    
        match sqlx::query_as::<_, Localite>(&self.localite_by_user_id_sql(&id).as_str())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in localite: {e:#?}");
                vec![]
            }
        }
    }

    fn localite_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT 
            l.id, 
            COALESCE(l.pays, '') as pays,
            COALESCE(l.ville, '') as ville,
            COALESCE(l.code_postal, '') as code_postal,
            COALESCE(l.commune, '') as commune,
            COALESCE(l.quartier, '') as quartier,
            COALESCE(l.adresse, '') as adresse,
            COALESCE(ul.consentement, 'false') as consentement
        FROM annuaire.localites l
        join annuaire.user_localites ul
        on l.id = ul.id_localite
        where ul.id_user = {id};
        "#)
    }
}