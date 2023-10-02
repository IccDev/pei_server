pub(crate) mod handlers;
pub(crate) mod responses;


use icc_common::{
    match_request::{match_request, Error, Params},
    hyper::{Request, Response, Body},
    futures::future::{BoxFuture},
    //multipart::server::FieldHeaders
};
use self::{
    responses::unknowed_route,
    handlers::user_handler,
};

// A boxed type definition for your async views.
pub type RouterHandler = Box<dyn Fn(Request<Body>, Params) -> BoxFuture<'static, Response<Body>> + Send + Sync>;


/*#[derive(Debug)]
pub(crate) struct FormData {
    pub(crate) headers: FieldHeaders,
    pub(crate) data: String
}*/


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
pub async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    let method = req.method();
    let path = req.uri().path();

    let (handler, params) = match_request!(method, path, {
        "/user/*" => {
            POST => crate::route_handler!(user_handler), 
            GET => crate::route_handler!(user_handler), 
        },
        "/*" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(unknowed_route),
        }
    })?;

    Ok(handler(req, params).await)
}