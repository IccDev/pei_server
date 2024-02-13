pub mod users;
pub mod annuaire;


use serde_derive::{Deserialize, Serialize};
use icc_common::{
    remoc::rch
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub data: MessageData,
    pub sender: rch::oneshot::Sender<Result<ResponseData, String>>
}

//###### Message to send ########################
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageData {
    Annuaire(self::annuaire::AnnuaireSearchInput)
}

//###### Message to receive #######################
#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseData {
    Annuaire(self::annuaire::AnnuaireSearchOutput)
}