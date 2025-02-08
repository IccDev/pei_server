use crate::router::match_request::Params;
use crate::addresses::mjib_address;


pub(crate) fn mjib_client(_params: Params) -> Result<String, String> {
    Ok(mjib_address())
}