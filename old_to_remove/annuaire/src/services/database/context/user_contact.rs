use crate::services::database::DatabaseService;
use common::sqlx;
use inter_services_messages::annuaire::{Contact, RowId};

impl DatabaseService {

    pub async fn contact_by_user_id(&self, user_id: &i32) -> Option<Contact> {
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

    pub async fn save_contact(&self, id_user: i32, contact: &Contact) -> i32 {
        match sqlx::query_as::<_, RowId>(&self.save_contact_sql(id_user, &contact))
        .fetch_one(&self.pool)
        .await 
        {
            Ok(res) => {
                match res.id {
                    Some(id) => id,
                    None => 0
                }
            },
            Err(e) => {
                println!("err in inserting user: {e:#?}");
                0
            }
        }
    }

    fn contact_sql(&self, user_id: &i32) -> String {
        format!(r#"SELECT uc.id, uc.gsm, COALESCE(uc.email, '') as email, COALESCE(uc.consentement_gsm, 'false') as consentement_gsm, COALESCE(uc.consentement_email, 'false') as consentement_email
        FROM annuaire.user_contact uc
        WHERE uc.id_user = {user_id};
        "#)
    }

    fn save_contact_sql(&self, id_user: i32, contact: &Contact) -> String {
        format!(r#"
            insert into annuaire.user_contact 
                (id_user, gsm, email, consentement_gsm, consentement_email) 
            values 
                ({}, '{}', '{}', '{}', '{}') 
            returning id
        "#,
        id_user,
        contact.gsm.clone().unwrap_or_default(),
        contact.email.clone().unwrap_or_default(),
        contact.consentement_gsm.clone().unwrap_or_default(),
        contact.consentement_email.clone().unwrap_or_default()
        )
    }
}