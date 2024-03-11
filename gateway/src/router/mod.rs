use common::{
    warp::{self, Filter}
};
use inter_services_messages::annuaire::AnnuaireSearchInput;
use crate::clients::annuaire_client::*;



pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
    .and(warp::path!("annuaire" / "search").and(warp::query::<AnnuaireSearchInput>()).then(annuaire_search))
    .or(warp::post().and(warp::path!("annuaire" / "create_usser").and(warp::body::json()).then(annuaire_create_user)))
    .or(warp::post().and(warp::path!("annuaire" / "create_campus").and(warp::body::json()).then(annuaire_create_campus)))
    .or(warp::post().and(warp::path!("annuaire" / "create_competences").and(warp::body::json()).then(annuaire_create_competences)))
    .or(warp::post().and(warp::path!("annuaire" / "create_departement").and(warp::body::json()).then(annuaire_create_departement)))
    .or(warp::post().and(warp::path!("annuaire" / "create_diplome").and(warp::body::json()).then(annuaire_create_diplome)))
    .or(warp::post().and(warp::path!("annuaire" / "create_domaine").and(warp::body::json()).then(annuaire_create_domaine)))
    .or(warp::post().and(warp::path!("annuaire" / "create_ecole").and(warp::body::json()).then(annuaire_create_ecole)))
    .or(warp::post().and(warp::path!("annuaire" / "create_entreprise").and(warp::body::json()).then(annuaire_create_entreprise)))
    .or(warp::post().and(warp::path!("annuaire" / "create_langue").and(warp::body::json()).then(annuaire_create_langue)))
    .or(warp::post().and(warp::path!("annuaire" / "create_localite").and(warp::body::json()).then(annuaire_create_localite)))
    .or(warp::post().and(warp::path!("annuaire" / "create_specialite").and(warp::body::json()).then(annuaire_create_specialite)))
    .or(warp::post().and(warp::path!("annuaire" / "create_titre").and(warp::body::json()).then(annuaire_create_titre)))
    .or(warp::post().and(warp::path!("annuaire" / "create_user_info").and(warp::body::json()).then(annuaire_create_user_info)))
}