/*
mod register_user;
pub mod user_data;
mod list_users;
mod login_user;
*/

use icc_common::{
    hyper::{Request, Response, Body},
    match_request::{match_request, Params}
};

use register_user::{register_user, not_user_route};
use list_users::list_users;
use login_user::login_user;


pub(crate) async fn user_handler(req: Request<Body>, _params: Params) -> Response<Body> {
    /*
        app.at("/register_user").post(register_user);
        app.at("/list_users").post(list_users);
        app.at("/login").post(login_user);


        app.at("/revoke_user").post(revoke_user);
        app.at("/get_user").post(get_user);
        app.at("/unrevoke_user").post(unrevoke_user);
        app.at("/update_user").post(update_user);
        app.at("/create_qr").post(create_qr);
        app.at("/create_totp").post(create_totp);
        app.at("/password_reset").post(password_reset);
    */
    let method = req.method();
    let path = req.uri().path();

    match match_request!(method, path, {
        "/user/register_user" => {
            POST => crate::route_handler!(register_user), 
        },
        "/user/list_users" => {
            GET => crate::route_handler!(list_users), 
        },
        "/user/login_user" => {
            POST => crate::route_handler!(login_user),
        },
        "/user/*" => {
            POST => crate::route_handler!(not_user_route),
        }
    }) {
        Ok((handler, par)) => handler(req, par).await,
        Err(_) => Response::new(Body::from(format!("User handler Error!")))
    }
}
