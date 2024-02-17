use common::{
    warp::{self, Filter}
};
use inter_services_messages::annuaire::AnnuaireSearchInput;
use crate::clients::annuaire_client::{annuaire_search, annuaire_register_post};

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
    .and(
        warp::path!("annuaire" / "search")
        .and(warp::query::<AnnuaireSearchInput>())
        .then(annuaire_search)
    )
    .or(
        warp::post()
        .and(
            warp::path!("annuaire" / "create")
            .and(warp::body::json())
            .and_then(annuaire_register_post)
        )
    )
}