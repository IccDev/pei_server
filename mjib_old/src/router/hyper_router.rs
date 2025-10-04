use common_crates::{
    hyper::{Request, Response, body::Incoming as IncomingBody, Result},
    futures::future::{BoxFuture}
};
use super::{
    unknowed_route,
    BoxedBody,
    match_request::Params,
    err,
    preflight
};
use crate::{
    match_request,
    routes::*,
    addresses::gateway_ip,
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
        "/mjib/query/branches_by_user/:user_id" => {
            GET => crate::route_handler!(query_branches_user_id_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/create/inscription" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_user_branches_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/branches" => {
            GET => crate::route_handler!(query_branches_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/annee_academique" => {
            GET => crate::route_handler!(query_annee_academique_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/disciplines" => {
            GET => crate::route_handler!(query_discipline_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/cours" => {
            GET => crate::route_handler!(query_cours_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/user/all" => {
            GET => crate::route_handler!(query_users_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/role/all" => {
            GET => crate::route_handler!(query_roles_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/church/all" => {
            GET => crate::route_handler!(query_churches_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/city/all" => {
            GET => crate::route_handler!(query_cities_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/query/country/all" => {
            GET => crate::route_handler!(query_countries_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/signup" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(signup_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/signin" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(signin_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/user/:user_id" => {
            GET => crate::route_handler!(signin_user_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/mjib/create/annee_academique" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_annee_academique_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/create/branches" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_branches_route),
            OPTIONS => crate::route_handler!(preflight),
        },

        "/mjib/create/cours" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_cours_route),
            OPTIONS => crate::route_handler!(preflight),
        },

        "/mjib/create/disciplines" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_discipline_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/create/roles" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_roles_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/create/churches" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_churches_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/create/cities" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_cities_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/create/countries" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(create_countries_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/mjib/*" => {
            GET => crate::route_handler!(unknowed_route), 
            POST => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
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