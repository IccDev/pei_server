use crate::router::match_request::Params;
use crate::{
    //result::GenericError,
    addresses::annuaire_address
};
//use std::convert::Infallible;


pub(crate) fn annuaire_client(_params: Params) -> Result<String, String> {
    Ok(annuaire_address())
}