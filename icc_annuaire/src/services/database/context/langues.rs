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
    annuaire::{AnnuaireSearch, Langue}
};


impl DatabaseService {

    pub(crate) async fn langues_by_user_id(&self, id: &i32) -> Vec<Langue> {
    
        match sqlx::query_as::<_, Langue>(&self.langues_by_user_id_sql(&id).as_str())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in langues: {e:#?}");
                vec![]
            }
        }
    }

    fn langues_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT l.id, l.nom, COALESCE(l.abbreviation, '') as abbreviation
        FROM annuaire.langues l
        join annuaire.user_langues ul
        on l.id = ul.id_langues
        where ul.id_user = {id};
        "#)
    }
}