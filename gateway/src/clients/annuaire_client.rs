use crate::router::match_request::Params;
use crate::addresses::annuaire_address;


pub(crate) fn annuaire_client(_params: Params) -> Result<String, String> {
    Ok(annuaire_address())
}