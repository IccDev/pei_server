pub mod annuaire;

use common::{
    serde::{self, Deserialize, Serialize},
    remoc::rch,
};
use self::annuaire::AnnuaireMessage;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Message {
    pub data: MessageData,
    pub sender: rch::oneshot::Sender<Result<ResponseData, String>>
}

//###### Message to send ########################
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum MessageData {
    Annuaire(AnnuaireMessage)
}

//###### Message to receive #######################
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub enum ResponseData {
    Annuaire(self::annuaire::AnnuaireSearchOutput)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Error {
    error: String
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error {
            error: error.to_string()
        }
    }
}