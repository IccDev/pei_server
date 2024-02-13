mod get_billets;
mod create_billet;

use icc_common::{
    hyper::{Request, Response, Body},
    match_request::{match_request, Params}
};
use get_billets::get_billets;
use create_billet::create_billet;
use crate::router::unknowned::unknowed_route;
use serde_derive::{Deserialize, Serialize};



pub(crate) async fn handle_co_voiturage(req: Request<Body>, _params: Params) -> Response<Body> {
    let method = req.method();
    let path = req.uri().path();

    match match_request!(method, path, {
        "/co-voiturage/get_billets" => {
            GET => crate::route_handler!(get_billets),
            POST => crate::route_handler!(unknowed_route),
        },
        "/co-voiturage/create_billet" => {
            POST => crate::route_handler!(create_billet), 
            GET => crate::route_handler!(unknowed_route),
        },
        "/co-voiturage/*" => {
            POST => crate::route_handler!(unknowed_route),
        }
    }) {
        Ok((handler, par)) => handler(req, par).await,
        Err(_) => Response::new(Body::from(format!("User handler Error!")))
    }
}
