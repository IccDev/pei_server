use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    sqlx
};
use inter_services_messages::{ResponseData, annuaire::{AnnuaireSearch, Contact}};


impl DatabaseService {

    pub(crate) async fn contact_by_user_id(&self, id: &i32) -> Option<Contact> {
        match sqlx::query_as::<_, Contact>(&self.contact_by_user_id_sql(&id).as_str())
        .fetch_one(&self.pool)
        .await 
        {
            Ok(res) => {
                Some(res)
            },
            Err(e) => {
                println!("err in contact: {e:#?}");
                //vec![]
                None
            }
        }
    }

    fn contact_by_user_id_sql(&self, id: &i32) -> String {
        format!(r#"
        SELECT uc.id, uc.gsm, COALESCE(uc.email, '') as email, COALESCE(uc.consentement_gsm, 'false') as consentement_gsm, COALESCE(uc.consentement_email, 'false') as consentement_email
        FROM annuaire.user_contact uc 
        where uc.id_user = {id};
        "#)
    }
}