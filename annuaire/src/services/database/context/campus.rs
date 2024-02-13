use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, Campus}};


impl DatabaseService {

    pub(crate) async fn campus_by_user_id(&self, id: &i32) -> Vec<Campus> {
        match sqlx::query_as::<_, Campus>(&self.campus_by_user_id_sql(&id).as_str())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in campus: {e:#?}");
                vec![]
            }
        }
    }

    fn campus_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT c.id, c.nom, COALESCE(c.description, '') as description
        FROM annuaire.campus c
        join annuaire.user_campus uc
        on c.id = uc.id_campus 
        where uc.id_user = {id};
        "#)
    }
}