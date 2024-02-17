use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{RowId, Contact};

impl DatabaseService {

    pub(crate) async fn contact_by_user_id(&self, user_id: &i32) -> Option<Contact> {
        match sqlx::query_as::<_, Contact>(&self.contact_sql(&user_id).as_ref())
        .fetch_all(&self.pool)
        .await 
        {
            Ok(res) => {
                res.first().cloned()
            },
            Err(e) => {
                println!("err in campus: {e:#?}");
                None
            }
        }
    }

    fn contact_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT uc.id, uc.gsm, COALESCE(uc.email, '') as email, COALESCE(uc.consentement_gsm, 'false') as consentement_gsm, COALESCE(uc.consentement_email, 'false') as consentement_email
        FROM annuaire.user_contact uc
        WHERE uc.id_user = {user_id};
        "#)
    }
}