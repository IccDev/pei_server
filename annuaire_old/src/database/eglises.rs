use common_crates::{
    tokio,
    serde_json, http_body_util::BodyExt,
    serde::de::Error,
    hyper::body::Buf
};
use crate::establish_connection;
use common_crates::{
    hyper::{Request, Response, body::Incoming as IncomingBody}
};
use crate::router::match_request::Params;
use crate::router::*;
use diesel::{select, insert_into, prelude::*};
use crate::schema::{adresses, eglises};
use crate::models::{Eglise, Adresse, EgliseData, EgliseNew, AdresseNew, AdresseData, AdresseInsert, EgliseInsert};
use super::get_request_body;
use std::time::SystemTime;

pub async fn annuaire_get_eglises(_req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    match eglises::table
        .left_join(adresses::table)
        .select((Eglise::as_select(), Option::<Adresse>::as_select()))
        .load::<(Eglise, Option<Adresse>)>(&mut establish_connection()) {
        Ok(data) => {
            let res: Vec<EgliseData> = data.iter().map(|(e,a)| EgliseData {
                id: e.id.to_owned(),
                nom: e.nom.to_owned(),
                description: e.description.to_owned(),
                created_at: e.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
                updated_at: e.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
                adresse: a.clone().map_or_else(|| AdresseData::default(), |d| AdresseData::from(&d)),
            })
            .collect();

            ok(BoxedBody::new(serde_json::json!(res).to_string()))
        },
        Err(e) => err(&format!("error: {e:#?}"))
    }
}

pub async fn annuaire_create_eglises(req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    match get_request_body::<Vec<EgliseNew>>(req).await {
        Ok(eglises_value) => {
            let mut conn = establish_connection();
            let Ok(now) = select(diesel::dsl::now).get_result::<SystemTime>(&mut conn) else {
                return err(&format!("error: not able to get current time from database!"));
            };
            
            // create adresses
            let adr: Vec<AdresseInsert> = eglises_value.iter().map(|e| AdresseInsert {
                pays: e.adresse.pays.clone(),
                ville: e.adresse.ville.clone(),
                commune: e.adresse.commune.clone(),
                code_postal: e.adresse.code_postal.clone(),
                rue: e.adresse.rue.clone(),
                numero: e.adresse.numero.clone(),
                boite: e.adresse.boite.clone(),
                created_at: Some(now.clone().into()),
                updated_at: Some(now.clone().into())
            }).collect();

            let created_adr: Vec<Adresse> = match insert_into(adresses::table)
                .values(adr.as_slice())
                .get_results(&mut conn) {
                    Ok(res) => res,
                    Err(e) => {
                        return err(&format!("error: {e:#?}"));
                    }
                };
            
            // create eglises
            let egl: Vec<EgliseInsert> = eglises_value.iter().map(|e| EgliseInsert {
                nom: e.nom.clone(),
                adresse_id: created_adr.iter().find(|c| 
                        &c.pays == &e.adresse.pays 
                        && &c.ville == &e.adresse.ville
                        && &c.rue == &e.adresse.rue)
                        .map_or_else(|| 0, |d| d.id.clone()),
                description: e.description.clone(),
                created_at: Some(now.clone().into()),
                updated_at: Some(now.into())
            }).collect();
            
            let created_egl: Vec<Eglise> = match insert_into(eglises::table)
                .values(egl.as_slice())
                .get_results(&mut conn) {
                    Ok(res) => res,
                    Err(e) => {
                        return err(&format!("error: {e:#?}"));
                    }
                };
            let results: Vec<EgliseData> = created_egl.iter().map(|e| {
                let mut created_eglise = EgliseData::from(e);
                created_eglise.adresse = created_adr.iter().find(|a| &a.id == &e.adresse_id).map_or_else(|| AdresseData::default(), |f| AdresseData::from(f));
                created_eglise
            })
            .collect();
            
            ok(BoxedBody::new(serde_json::json!(results).to_string()))
        },
        Err(e) => err(&format!("error: {e:#?}"))
    }
}