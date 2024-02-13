mod search;

use crate::router::responses::{unknowed_route, err};
use icc_common::{
    hyper::{Request, Response, Body},
    match_request::{match_request, Params}
};
use self::{
    search::handle_search
};



pub(crate) async fn annuaire_handler(req: Request<Body>, _params: Params) -> Response<Body> {

    let method = req.method();
    let path = req.uri().path();

    match match_request!(method, path, {
        "/annuaire/search/" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(handle_search),
        },
        "/annuaire/*" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(unknowed_route),
        }
    }) {
        Ok((handler, par)) => handler(req, par).await,
        Err(_) => err("Annuaire handler Error!")
    }
}
