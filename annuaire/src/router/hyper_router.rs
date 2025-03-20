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
    routes::*,
    addresses::gateway_ip,
    router::preflight
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
        "/annuaire/create/user" => {
            POST => crate::route_handler!(create_user_route),
            GET => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(preflight),
        },
        "/annuaire/query/users/:key/:church" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(query_users_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/annuaire/query/user/:id/contact" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(query_user_to_contact_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/annuaire/query/user/:id" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(query_user_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/annuaire/query/all/churches" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(query_churches_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        },
        "/*" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(unknowed_route),
            OPTIONS => crate::route_handler!(unknowed_route),
        }
    }) {
        Ok((handler, params)) => Ok(handler(req, params).await),
        Err(_) => Ok(err("No route found"))
    }
}