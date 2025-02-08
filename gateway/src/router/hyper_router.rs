use common_crates::hyper::Method;
use super::{
    unknowned::unknowed_route,
    match_request::Params
};
use crate::{
    match_request, 
    clients::*
};


// A boxed type definition for your async views.
pub type RouterHandler = Box<dyn Fn(Params) -> Result<String, String>>;


#[macro_export]
macro_rules! route_handler {
    ($closure:expr) => {{
        #[allow(unused_mut)]
        let mut closure = $closure;
        let b: crate::router::RouterHandler
         = Box::new(move |params| {
            closure(params)
        });
        b
    }};
}

// An example request router.
pub async fn router(path: &str, method: Method) -> String {
    match match_request!(method, path, {
        "/annuaire/*" => {
            GET => crate::route_handler!(annuaire_client), 
            POST => crate::route_handler!(annuaire_client),
            OPTIONS => crate::route_handler!(annuaire_client),
        },
        "/mjib/*" => {
            GET => crate::route_handler!(mjib_client), 
            POST => crate::route_handler!(mjib_client),
            OPTIONS => crate::route_handler!(mjib_client),
        },
        "/*" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(unknowed_route),
        }
    }) {
        Ok((handler, params)) => handler(params).map_or_else(|_| "127.0.0.0:2222".to_string(), |r| r),
        Err(_) => "127.0.0.0:2222".to_string()
    }

}