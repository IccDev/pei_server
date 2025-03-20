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
use crate::schema::*;
use crate::models::*;
use super::get_request_body;
use std::{time::SystemTime, mem::swap};

pub async fn get_infos_to_create_user(_req: Request<IncomingBody>, _params: Params) -> Response<BoxedBody> {
    let mut infos = InfosToCreateUser::default();

    let mut eglises = match eglises::table
        .left_join(adresses::table)
        .select((Eglise::as_select(), Option::<Adresse>::as_select()))
        .load::<(Eglise, Option<Adresse>)>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|(e,a)| EgliseData {
                id: e.id.to_owned(),
                nom: e.nom.to_owned(),
                description: e.description.to_owned(),
                created_at: e.created_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
                updated_at: e.updated_at.map_or_else(|| None, |d| Some(format!("{}", d.format("%d-%m-%Y %H:%M")))),
                adresse: a.clone().map_or_else(|| AdresseData::default(), |d| AdresseData::from(&d)),
            })
            .collect()
        },
        Err(e) => vec![]
    };

    let mut langues = match langues::table
        .select(Langue::as_select())
        .load::<Langue>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|d| LangueData::from(d))
            .collect()
        },
        Err(e) => vec![]
    };

    let mut departements = match departements::table
        .select(Departement::as_select())
        .load::<Departement>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|d| DepartementData::from(d))
            .collect()
        },
        Err(e) => vec![]
    };
    let mut eglise_departements = match eglise_departements::table
        .select(EgliseDepartement::as_select())
        .load::<EgliseDepartement>(&mut establish_connection()) {
            Ok(data) => {
                data.iter().map(|d| EgliseDepartementData::from(d))
                .collect()
            },
            Err(e) => vec![]
        };
    
    let mut composants = match composants::table
        .select(Composant::as_select())
        .load::<Composant>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|d| ComposantData::from(d))
            .collect()
        },
        Err(e) => vec![]
    };

    let mut qualifications = match qualifications::table
        .select(Qualification::as_select())
        .load::<Qualification>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|d| QualificationData::from(d))
            .collect()
        },
        Err(e) => vec![]
    };

    let mut infos_qualifications = match infos_qualifications::table
        .select(InfosQualification::as_select())
        .load::<InfosQualification>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|d| InfosQualificationData::from(d))
            .collect()
        },
        Err(e) => vec![]
    };

    let mut infos_composants = match infos_composants::table
        .select(InfosComposant::as_select())
        .load::<InfosComposant>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|d| InfosComposantData::from(d))
            .collect()
        },
        Err(e) => vec![]
    };

    let mut parcours = match parcours::table
        .select(Parcours::as_select())
        .load::<Parcours>(&mut establish_connection()) {
        Ok(data) => {
            data.iter().map(|d| ParcoursData::from(d))
            .collect()
        },
        Err(e) => vec![]
    };

    swap(&mut infos.eglises, &mut eglises);
    swap(&mut infos.langues, &mut langues);
    swap(&mut infos.departements, &mut departements);
    swap(&mut infos.eglise_departements, &mut eglise_departements);
    swap(&mut infos.composants, &mut composants);
    swap(&mut infos.qualifications, &mut qualifications);
    swap(&mut infos.infos_qualifications, &mut infos_qualifications);
    swap(&mut infos.infos_composants, &mut infos_composants);
    swap(&mut infos.parcours, &mut parcours);

    ok(BoxedBody::new(serde_json::json!(infos).to_string()))
}
