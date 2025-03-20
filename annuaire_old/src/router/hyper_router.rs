use common_crates::{
    hyper::{Request, Response, body::Incoming as IncomingBody, Result},
    futures::future::{BoxFuture}
};
use super::{
    unknowed_route,
    BoxedBody,
    match_request::Params,
    err
};
use crate::{
    match_request,
    database::*
};


// A boxed type definition for your async views.
pub type RouterHandler = Box<dyn Fn(Request<IncomingBody>, Params) -> BoxFuture<'static, Response<BoxedBody>> + Send + Sync>;


#[macro_export]
macro_rules! route_handler {
    ($closure:expr) => {{
        #[allow(unused_mut)]
        let mut closure = $closure;
        let b: crate::router::RouterHandler
         = Box::new(move |req, params| {
            Box::pin(closure(req, params))
        });
        b
    }};
}



// An example request router.
pub async fn router(req: Request<IncomingBody>) -> Result<Response<BoxedBody>> {
    let method = req.method();
    let path = req.uri().path();
    match match_request!(method, path, {
        "/annuaire/eglises" => {
            GET => crate::route_handler!(annuaire_get_eglises), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/annuaire/get_infos_to_create_user" => {
            GET => crate::route_handler!(get_infos_to_create_user), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/annuaire/create_eglises" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(annuaire_create_eglises),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        /*
        "/annuaire/search" => {
            GET => crate::route_handler!(annuaire_search), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/annuaire/create_user" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(annuaire_create_user),
            OPTIONS => crate::route_handler!(annuaire_create_user),
        },
        "/annuaire/get_infos_to_create_user" => {
            GET => crate::route_handler!(annuaire_get_infos_to_create_user), 
            POST => crate::route_handler!(unknowed_route), 
            OPTIONS => crate::route_handler!(annuaire_get_infos_to_create_user),
        },
        "/annuaire/get_user_by_id/:user_id" => {
            GET => crate::route_handler!(annuaire_get_user_by_id), 
            POST => crate::route_handler!(unknowed_route), 
            OPTIONS => crate::route_handler!(annuaire_get_user_by_id),
        },
        "/annuaire/create_eglises" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(annuaire_create_eglises),
            OPTIONS => crate::route_handler!(annuaire_create_eglises),
        },
        "/annuaire/create_departements" => {
            POST => crate::route_handler!(annuaire_create_departements), 
            GET => crate::route_handler!(unknowed_route), 
            OPTIONS => crate::route_handler!(annuaire_create_departements),
        },
        "/annuaire/create_langues" => {
            GET => crate::route_handler!(unknowed_route),
            POST => crate::route_handler!(annuaire_create_langues),
            OPTIONS => crate::route_handler!(annuaire_create_langues),
        },
        "/annuaire/create_certificats" => {
            GET => crate::route_handler!(unknowed_route),
            POST => crate::route_handler!(annuaire_create_certificats),
            OPTIONS => crate::route_handler!(annuaire_create_certificats),
        },
        "/annuaire/link_eglise_departements" => {
            GET => crate::route_handler!(unknowed_route),
            POST => crate::route_handler!(link_eglise_departements),
            OPTIONS => crate::route_handler!(link_eglise_departements),
        },
        */
        "/" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        }
    }) {
        Ok((handler, params)) => Ok(handler(req, params).await),
        Err(_) => Ok(err("No route found"))
    }
}