use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, Competence}};


impl DatabaseService {

    pub(crate) async fn competence_by_user_id(&self, id: &i32) -> Vec<Competence> {
        match sqlx::query_as::<_, Competence>(&self.competence_by_user_id_sql(&id).as_str())
        .fetch_all(&self.pool)
        .await
        {
            Ok(res) => {
                res.to_vec()
            },
            Err(e) => {
                println!("err in competences: {e:#?}");
                vec![]
            }
        }
    }

    fn competence_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT c.id, c.nom, COALESCE(c.description, '') as description
        FROM annuaire.competences c
        join annuaire.user_competences uc
        on c.id = uc.id_competence  
        where uc.id_user = {id};
        "#)
    }
}