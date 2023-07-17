use crate::{
    services::{
        database::DatabaseService
    }
};
use icc_common::{
    acteur::{Serve, ServiceAssistant},
    async_trait,
    sqlx,
    time::OffsetDateTime,
    uuid::Uuid
};
use inter_services_messages::{covoiturage::Billet, ResponseData, CovoiturageResponseData};


#[async_trait::async_trait]
impl Serve<Billet> for DatabaseService {
    type Response = Result<ResponseData, String>;

    async fn handle(&self, message: Billet, _system: &ServiceAssistant<Self>) -> Self::Response {
        self.create_billet(message).await
    }
}


impl DatabaseService {
    pub(crate) async fn create_billet(&self, message: Billet) -> Result<ResponseData, String> {
        //TODO: Check if the email really exist!

        let id = Uuid::new_v4();
        let created_at = OffsetDateTime::now_utc();

        let creator_email = match message.creator_email {
            Some(data) => data.clone(),
            None => "".to_owned()
        };

        let lieu_depart = match message.lieu_depart {
            Some(data) => data.clone(),
            None => "".to_owned()
        };

        let lieu_destination = match message.lieu_destination {
            Some(data) => data.clone(),
            None => "".to_owned()
        };
        
        let points_stib = message.points_stib.join("@");

        match sqlx::query(
            format!(
                r#"INSERT INTO icc.billets ( 
                    id,
                    creator,
                    creator_email,
                    places,
                    lieu_depart,
                    points_stib,
                    lieu_destination,
                    activated,
                    date_depart,
                    created_at
                )
                VALUES ( '{}', '{}', '{}', {}, '{}', '{}', '{}', {}, '{}', '{}' );
                "#,
                    &id,
                    &message.creator,
                    &creator_email,
                    &message.places,
                    &lieu_depart,
                    &points_stib,
                    &lieu_destination,
                    true,
                    &message.date_depart,
                    &created_at
    
            ).as_str())
            .execute(&self.pool)
            .await 
        {
            Ok(_) => Ok(ResponseData::Covoiturage(CovoiturageResponseData::CreateBillet(id.to_string()))),
            Err(e) => {
                // need to check well before sending the error message back
                println!("Error: {:#?}", e);
                Err("not able to register!".to_owned())
            }
        }
    }
}